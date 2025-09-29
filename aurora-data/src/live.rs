use anyhow::{Result, anyhow};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{info, error, debug, warn};
use aurora_core::Kline;

/// 接收实时数据流
pub async fn stream_data(symbol: &str, stream_type: &str, interval: &str) -> Result<()> {
    let stream_name = match stream_type {
        "kline" => format!("{}@kline_{}", symbol.to_lowercase(), interval),
        "trade" => format!("{}@trade", symbol.to_lowercase()),
        _ => return Err(anyhow!("不支持的流类型: {}", stream_type)),
    };
    
    let url = format!("wss://stream.binance.com:9443/ws/{}", stream_name);
    info!("连接到WebSocket: {}", url);
    
    loop {
        match connect_and_stream(&url, stream_type).await {
            Ok(_) => {
                info!("WebSocket连接正常结束");
                break;
            }
            Err(e) => {
                error!("WebSocket连接错误: {}", e);
                info!("5秒后尝试重连...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
    
    Ok(())
}

async fn connect_and_stream(url: &str, stream_type: &str) -> Result<()> {
    let (ws_stream, _) = connect_async(url).await?;
    info!("WebSocket连接成功");
    
    let (mut write, mut read) = ws_stream.split();
    
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                debug!("收到消息: {}", text);
                
                match stream_type {
                    "kline" => {
                        if let Err(e) = process_kline_message(&text) {
                            error!("处理K线消息失败: {}", e);
                        }
                    }
                    "trade" => {
                        if let Err(e) = process_trade_message(&text) {
                            error!("处理交易消息失败: {}", e);
                        }
                    }
                    _ => warn!("未知的流类型: {}", stream_type),
                }
            }
            Ok(Message::Close(_)) => {
                info!("WebSocket连接被关闭");
                break;
            }
            Ok(Message::Ping(ping)) => {
                debug!("收到Ping，发送Pong");
                if let Err(e) = write.send(Message::Pong(ping)).await {
                    error!("发送Pong失败: {}", e);
                }
            }
            Ok(_) => {
                // 忽略其他类型的消息
            }
            Err(e) => {
                error!("WebSocket消息错误: {}", e);
                return Err(e.into());
            }
        }
    }
    
    Ok(())
}

/// 处理K线消息
fn process_kline_message(text: &str) -> Result<()> {
    let value: Value = serde_json::from_str(text)?;
    
    if let Some(kline_data) = value.get("k") {
        let kline = Kline {
            timestamp: kline_data["t"].as_i64().unwrap_or(0),
            open: kline_data["o"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            high: kline_data["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low: kline_data["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            close: kline_data["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume: kline_data["v"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
        };
        
        // 只处理完成的K线
        if kline_data["x"].as_bool().unwrap_or(false) {
            info!("📊 完成的K线: 时间={}, 开盘={}, 最高={}, 最低={}, 收盘={}, 成交量={}", 
                  kline.timestamp, kline.open, kline.high, kline.low, kline.close, kline.volume);
        } else {
            debug!("🔄 实时K线更新: 时间={}, 当前价格={}, 成交量={}", 
                   kline.timestamp, kline.close, kline.volume);
        }
    }
    
    Ok(())
}

/// 处理交易消息
fn process_trade_message(text: &str) -> Result<()> {
    let value: Value = serde_json::from_str(text)?;
    
    let price: f64 = value["p"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
    let quantity: f64 = value["q"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
    let timestamp: i64 = value["T"].as_i64().unwrap_or(0);
    let is_buyer_maker = value["m"].as_bool().unwrap_or(false);
    
    let side = if is_buyer_maker { "卖单" } else { "买单" };
    
    info!("💱 实时交易: 价格={}, 数量={}, 时间={}, 方向={}", 
          price, quantity, timestamp, side);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_kline_message() {
        let test_message = r#"{
            "e": "kline",
            "E": 123456789,
            "s": "BTCUSDT",
            "k": {
                "t": 1640995200000,
                "T": 1640995259999,
                "s": "BTCUSDT",
                "i": "1m",
                "f": 100,
                "L": 200,
                "o": "50000.00",
                "c": "50500.00",
                "h": "51000.00",
                "l": "49000.00",
                "v": "100.0",
                "n": 100,
                "x": true,
                "q": "5050000.0",
                "V": "50.0",
                "Q": "2525000.0"
            }
        }"#;
        
        let result = process_kline_message(test_message);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_process_trade_message() {
        let test_message = r#"{
            "e": "trade",
            "E": 123456789,
            "s": "BTCUSDT",
            "t": 12345,
            "p": "50000.00",
            "q": "0.001",
            "b": 88,
            "a": 50,
            "T": 1640995200000,
            "m": false,
            "M": true
        }"#;
        
        let result = process_trade_message(test_message);
        assert!(result.is_ok());
    }
}