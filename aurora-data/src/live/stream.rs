//! # 实时数据流实现
//!
//! 包含了Binance实时数据流的具体实现。

use crate::{DataError, DataResult, DataSourceConfig};
use async_trait::async_trait;
use aurora_core::{DataSource, Kline, MarketEvent};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::collections::VecDeque;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
use tracing::{debug, error, info, warn};

/// Binance实时数据流
///
/// 这个结构体提供了从Binance WebSocket获取实时市场数据的功能。
/// 支持K线数据的实时订阅和自动重连。
///
/// ## 功能特性
///
/// - **WebSocket连接**: 维持与交易所的WebSocket连接
/// - **实时K线**: 获取实时更新的K线数据
/// - **自动重连**: 连接断开时自动重连
/// - **数据过滤**: 只处理完成的K线数据
/// - **错误处理**: 完整的连接和数据错误处理
///
/// ## 使用示例
///
/// ```rust,no_run
/// use aurora_data::BinanceLiveStream;
/// use aurora_core::DataSource;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut stream = BinanceLiveStream::new();
///
/// // 连接到实时数据流
/// stream.connect(&["BTCUSDT", "ETHUSDT"]).await?;
///
/// // 接收数据
/// while let Some(kline) = stream.next_kline().await? {
///     println!("收到K线: 价格 {}, 成交量 {}", kline.close, kline.volume);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct BinanceLiveStream {
    /// 数据源配置
    config: DataSourceConfig,

    /// WebSocket连接（可选，因为可能未连接或连接断开）
    ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,

    /// 订阅的交易对列表
    subscribed_symbols: Vec<String>,

    /// K线数据缓存队列
    kline_buffer: VecDeque<Kline>,

    /// 是否处于连接状态
    is_connected: bool,

    /// 重连计数器
    reconnect_count: u32,

    /// 时间间隔设置
    interval: String,
}

impl BinanceLiveStream {
    /// 创建新的Binance实时数据流实例
    ///
    /// 使用默认配置创建实例，包括：
    /// - 默认WebSocket URL
    /// - 自动重连设置
    /// - 1分钟K线间隔
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_data::BinanceLiveStream;
    ///
    /// let stream = BinanceLiveStream::new();
    /// ```
    pub fn new() -> Self {
        Self::with_config(DataSourceConfig::default())
    }

    /// 使用指定配置创建实时数据流实例
    ///
    /// # 参数
    ///
    /// * `config` - 数据源配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_data::{BinanceLiveStream, DataSourceConfig};
    ///
    /// let config = DataSourceConfig::new("https://api.binance.com")
    ///     .with_websocket("wss://stream.binance.com:9443");
    /// let stream = BinanceLiveStream::with_config(config);
    /// ```
    pub fn with_config(config: DataSourceConfig) -> Self {
        Self {
            config,
            ws_stream: None,
            subscribed_symbols: Vec::new(),
            kline_buffer: VecDeque::new(),
            is_connected: false,
            reconnect_count: 0,
            interval: "1m".to_string(), // 默认1分钟K线
        }
    }

    /// 设置K线时间间隔
    ///
    /// # 参数
    ///
    /// * `interval` - 时间间隔，如"1m", "5m", "1h", "1d"
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_data::BinanceLiveStream;
    ///
    /// let mut stream = BinanceLiveStream::new();
    /// stream.set_interval("5m");
    /// ```
    pub fn set_interval(&mut self, interval: &str) {
        self.interval = interval.to_string();
    }

    /// 连接到WebSocket并订阅指定交易对
    ///
    /// 这个方法会建立WebSocket连接，并订阅指定交易对的K线数据。
    ///
    /// # 参数
    ///
    /// * `symbols` - 要订阅的交易对列表
    ///
    /// # 返回值
    ///
    /// 成功时返回()，失败时返回DataError
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use aurora_data::BinanceLiveStream;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut stream = BinanceLiveStream::new();
    /// stream.connect(&["BTCUSDT", "ETHUSDT"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(&mut self, symbols: &[&str]) -> DataResult<()> {
        // 构建WebSocket URL，支持多个交易对
        let ws_url = self
            .config
            .ws_url
            .as_ref()
            .ok_or_else(|| DataError::ConfigError("WebSocket URL未配置".to_string()))?;

        // 构建流名称列表
        let streams: Vec<String> = symbols
            .iter()
            .map(|symbol| format!("{}@kline_{}", symbol.to_lowercase(), self.interval))
            .collect();

        let stream_params = streams.join("/");
        let url = format!("{}/stream?streams={}", ws_url, stream_params);

        info!("连接到WebSocket: {}", url);
        info!("订阅的交易对: {:?}, 间隔: {}", symbols, self.interval);

        // 建立WebSocket连接
        let (ws_stream, _) = connect_async(&url)
            .await
            .map_err(|e| DataError::WebSocketError(format!("连接失败: {}", e)))?;

        self.ws_stream = Some(ws_stream);
        self.subscribed_symbols = symbols.iter().map(|s| s.to_uppercase()).collect();
        self.is_connected = true;
        self.reconnect_count = 0;

        info!("WebSocket连接成功，已订阅 {} 个交易对", symbols.len());
        Ok(())
    }

    /// 获取下一个K线数据
    ///
    /// 这个方法会从WebSocket连接读取下一条消息，解析为K线数据并返回。
    /// 如果连接断开，会自动尝试重连。
    ///
    /// # 返回值
    ///
    /// * `Some(Kline)` - 成功获取到K线数据
    /// * `None` - 没有数据或连接已关闭
    /// * `Err(DataError)` - 发生错误
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use aurora_data::BinanceLiveStream;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut stream = BinanceLiveStream::new();
    /// stream.connect(&["BTCUSDT"]).await?;
    ///
    /// while let Some(kline) = stream.next_kline().await? {
    ///     println!("价格: {}, 成交量: {}", kline.close, kline.volume);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn next_kline(&mut self) -> DataResult<Option<Kline>> {
        // 首先检查缓存中是否有数据
        if let Some(kline) = self.kline_buffer.pop_front() {
            return Ok(Some(kline));
        }

        // 如果没有连接，返回None
        if !self.is_connected || self.ws_stream.is_none() {
            return Ok(None);
        }

        // 从WebSocket读取消息
        loop {
            let message = {
                let ws_stream = self.ws_stream.as_mut().unwrap();
                match ws_stream.next().await {
                    Some(Ok(message)) => message,
                    Some(Err(e)) => {
                        error!("WebSocket消息错误: {}", e);
                        self.handle_connection_error().await?;
                        return Ok(None);
                    }
                    None => {
                        warn!("WebSocket连接已关闭");
                        self.handle_connection_error().await?;
                        return Ok(None);
                    }
                }
            };

            match message {
                Message::Text(text) => {
                    debug!("收到WebSocket消息: {}", text);

                    if let Ok(kline) = self.parse_kline_message(&text) {
                        return Ok(Some(kline));
                    }
                    // 如果解析失败，继续等待下一条消息
                }
                Message::Close(_) => {
                    info!("收到WebSocket关闭消息");
                    self.handle_connection_error().await?;
                    return Ok(None);
                }
                Message::Ping(ping) => {
                    debug!("收到Ping，发送Pong");
                    let ws_stream = self.ws_stream.as_mut().unwrap();
                    if let Err(e) = ws_stream.send(Message::Pong(ping)).await {
                        error!("发送Pong失败: {}", e);
                    }
                }
                _ => {
                    // 忽略其他类型的消息
                }
            }
        }
    }

    /// 检查连接状态
    ///
    /// # 返回值
    ///
    /// 如果当前已连接返回true，否则返回false
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_data::BinanceLiveStream;
    ///
    /// let stream = BinanceLiveStream::new();
    /// assert!(!stream.is_connected());
    /// ```
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    /// 获取已订阅的交易对列表
    ///
    /// # 返回值
    ///
    /// 返回已订阅的交易对符号列表
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_data::BinanceLiveStream;
    ///
    /// let stream = BinanceLiveStream::new();
    /// let symbols = stream.subscribed_symbols();
    /// assert!(symbols.is_empty()); // 初始状态下没有订阅
    /// ```
    pub fn subscribed_symbols(&self) -> &[String] {
        &self.subscribed_symbols
    }

    /// 断开WebSocket连接
    ///
    /// 主动断开当前的WebSocket连接，清理相关资源。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_data::BinanceLiveStream;
    ///
    /// let mut stream = BinanceLiveStream::new();
    /// stream.disconnect();
    /// assert!(!stream.is_connected());
    /// ```
    pub fn disconnect(&mut self) {
        self.ws_stream = None;
        self.is_connected = false;
        self.kline_buffer.clear();
        info!("WebSocket连接已断开");
    }

    /// 处理连接错误和重连逻辑
    ///
    /// 当连接出现错误时，这个方法会执行重连逻辑。
    /// 包括指数退避延迟和最大重试次数限制。
    async fn handle_connection_error(&mut self) -> DataResult<()> {
        self.is_connected = false;
        self.ws_stream = None;

        if self.reconnect_count >= self.config.max_retries {
            return Err(DataError::WebSocketError(format!(
                "达到最大重连次数 {}",
                self.config.max_retries
            )));
        }

        self.reconnect_count += 1;
        let delay_secs = std::cmp::min(300, 5 * (1 << (self.reconnect_count - 1))); // 指数退避，最大5分钟

        warn!(
            "连接断开，{}秒后进行第{}次重连",
            delay_secs, self.reconnect_count
        );
        tokio::time::sleep(Duration::from_secs(delay_secs)).await;

        // 克隆订阅信息以避免借用问题
        let symbols_owned: Vec<String> = self.subscribed_symbols.clone();
        let symbols: Vec<&str> = symbols_owned.iter().map(|s| s.as_str()).collect();
        if !symbols.is_empty() {
            self.connect(&symbols).await?;
        }

        Ok(())
    }

    /// 解析WebSocket消息为K线数据
    ///
    /// 将接收到的JSON消息解析为Kline结构体。
    /// 只处理完成的K线数据（x字段为true）。
    ///
    /// # 参数
    ///
    /// * `text` - JSON格式的WebSocket消息
    ///
    /// # 返回值
    ///
    /// 成功时返回Kline，失败时返回DataError
    fn parse_kline_message(&self, text: &str) -> DataResult<Kline> {
        let value: Value = serde_json::from_str(text)
            .map_err(|e| DataError::ParseError(format!("JSON解析失败: {}", e)))?;

        // 处理单个流的数据格式
        let kline_data = if let Some(data) = value.get("data") {
            // 多流格式: {"stream": "btcusdt@kline_1m", "data": {...}}
            data.get("k")
                .ok_or_else(|| DataError::ParseError("消息中缺少K线数据".to_string()))?
        } else if let Some(k) = value.get("k") {
            // 单流格式: {"e": "kline", "k": {...}}
            k
        } else {
            return Err(DataError::ParseError("无法识别的消息格式".to_string()));
        };

        // 检查是否为完成的K线
        let is_closed = kline_data["x"].as_bool().unwrap_or(false);
        if !is_closed {
            return Err(DataError::ParseError("K线尚未完成".to_string()));
        }

        // 解析K线数据
        let kline = Kline {
            timestamp: kline_data["t"].as_i64().unwrap_or(0),
            open: kline_data["o"]
                .as_str()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0.0),
            high: kline_data["h"]
                .as_str()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0.0),
            low: kline_data["l"]
                .as_str()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0.0),
            close: kline_data["c"]
                .as_str()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0.0),
            volume: kline_data["v"]
                .as_str()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0.0),
        };

        // 验证数据有效性
        if !super::utils::validate_kline(&kline) {
            return Err(DataError::ParseError("K线数据验证失败".to_string()));
        }

        info!(
            "📊 收到完成的K线: 时间={}, 开盘={}, 最高={}, 最低={}, 收盘={}, 成交量={}",
            kline.timestamp, kline.open, kline.high, kline.low, kline.close, kline.volume
        );

        Ok(kline)
    }
}

#[async_trait]
impl DataSource for BinanceLiveStream {
    /// 启动数据源并返回事件接收器
    ///
    /// 这个方法启动WebSocket连接，并返回一个接收器来接收市场事件。
    ///
    /// # 返回值
    ///
    /// 返回一个用于接收MarketEvent的UnboundedReceiver
    ///
    /// # 错误
    ///
    /// 如果WebSocket连接失败或配置错误，返回anyhow::Error
    async fn start(&mut self) -> anyhow::Result<UnboundedReceiver<MarketEvent>> {
        use tokio::sync::mpsc;

        let (tx, rx) = mpsc::unbounded_channel();

        // 如果还没有连接，使用默认的BTCUSDT
        if self.subscribed_symbols.is_empty() {
            self.connect(&["BTCUSDT"])
                .await
                .map_err(|e| anyhow::anyhow!("连接失败: {}", e))?;
        }

        // 创建独立的实例用于后台任务，避免生命周期问题
        let config = self.config.clone();
        let interval = self.interval.clone();
        let symbols_owned: Vec<String> = self.subscribed_symbols.clone();

        tokio::spawn(async move {
            let mut stream_clone = BinanceLiveStream::with_config(config);
            stream_clone.interval = interval;
            let symbols: Vec<&str> = symbols_owned.iter().map(|s| s.as_str()).collect();

            if let Err(e) = stream_clone.connect(&symbols).await {
                error!("后台任务连接失败: {}", e);
                return;
            }

            loop {
                match stream_clone.next_kline().await {
                    Ok(Some(kline)) => {
                        let event = MarketEvent::Kline(kline);
                        if let Err(_) = tx.send(event) {
                            info!("事件接收器已关闭，停止数据流");
                            break;
                        }
                    }
                    Ok(None) => {
                        debug!("没有更多数据");
                        break;
                    }
                    Err(e) => {
                        error!("获取K线数据失败: {}", e);
                        // 可以选择继续或者退出
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        });

        Ok(rx)
    }
}

#[cfg(test)]
mod tests; // 外部测试模块
