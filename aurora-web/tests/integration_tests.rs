// Copyright 2025 blingbling21
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Aurora Web 集成测试

#[cfg(test)]
mod tests {
    use aurora_web::api;
    use aurora_web::state::AppState;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tower::ServiceExt; // for `oneshot`

    /// 创建测试应用
    fn create_test_app() -> Router {
        let app_state = AppState {
            backtest_tasks: Arc::new(RwLock::new(std::collections::HashMap::new())),
            data_dir: std::path::PathBuf::from("./test_data"),
            config_dir: std::path::PathBuf::from("./test_configs"),
        };

        // 确保测试目录存在
        std::fs::create_dir_all(&app_state.data_dir).ok();
        std::fs::create_dir_all(&app_state.config_dir).ok();

        Router::new()
            .nest("/api/config", api::config::routes())
            .nest("/api/backtest", api::backtest::routes())
            .nest("/api/data", api::data::routes())
            .with_state(app_state)
    }

    #[tokio::test]
    async fn test_list_configs() {
        let app = create_test_app();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/config")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_config() {
        let app = create_test_app();

        let config_data = serde_json::json!({
            "filename": "test.toml",
            "content": "[portfolio]\ninitial_cash = 10000.0\n"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/config")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&config_data).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(
            response.status() == StatusCode::OK
                || response.status() == StatusCode::BAD_REQUEST
                || response.status() == StatusCode::CONFLICT
        );
    }

    #[tokio::test]
    async fn test_list_data_files() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/data/list")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_backtests() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/backtest")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_validate_config() {
        let app = create_test_app();

        let valid_config = serde_json::json!({
            "content": "[portfolio]\ninitial_cash = 10000.0\ncommission = 0.001\n"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/config/validate")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&valid_config).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_nonexistent_config() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/config/nonexistent.toml")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_backtest_task() {
        let app = create_test_app();

        let task_data = serde_json::json!({
            "name": "测试回测",
            "config_path": "./test_configs/test.toml",
            "data_path": "./test_data/test.csv"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/backtest")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&task_data).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_klines_data() {
        // 创建测试数据文件
        let test_data_dir = std::path::PathBuf::from("./test_data");
        std::fs::create_dir_all(&test_data_dir).ok();
        
        let test_csv_path = test_data_dir.join("test_klines.csv");
        let test_csv_content = "timestamp,open,high,low,close,volume
1640995200000,47000.0,47500.0,46800.0,47200.0,1000.0
1640998800000,47200.0,47600.0,47000.0,47400.0,1200.0
1641002400000,47400.0,47800.0,47300.0,47600.0,1100.0";
        
        std::fs::write(&test_csv_path, test_csv_content).unwrap();
        
        let app = create_test_app();

        // 测试加载K线数据
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/data/klines?path=test_klines.csv")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        
        // 验证响应内容
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(json["success"], true);
        assert!(json["data"].is_array());
        assert_eq!(json["data"].as_array().unwrap().len(), 3);
        
        // 清理测试文件
        std::fs::remove_file(&test_csv_path).ok();
    }

    #[tokio::test]
    async fn test_get_klines_nonexistent_file() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/data/klines?path=nonexistent.csv")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // 应该返回错误状态
        assert_ne!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_klines_invalid_extension() {
        // 创建非CSV文件
        let test_data_dir = std::path::PathBuf::from("./test_data");
        std::fs::create_dir_all(&test_data_dir).ok();
        
        let test_file_path = test_data_dir.join("test.txt");
        std::fs::write(&test_file_path, "not a csv").unwrap();
        
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/data/klines?path=test.txt")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // 应该返回错误状态
        assert_ne!(response.status(), StatusCode::OK);
        
        // 清理测试文件
        std::fs::remove_file(&test_file_path).ok();
    }
}
