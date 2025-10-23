// Aurora Web - 交易点位图表可视化（使用 Lightweight Charts）

let tradingChart = null;
let candlestickSeries = null;
let buyMarkers = [];
let sellMarkers = [];

/**
 * 渲染交易点位图
 * @param {Array} klineData - K线数据 [{timestamp, open, high, low, close, volume}]
 * @param {Array} trades - 交易记录 [{timestamp, price, quantity, is_buy}]
 */
function renderTradingChart(klineData, trades) {
    const container = document.getElementById('trading-chart');
    if (!container) {
        console.error('找不到交易图表容器');
        return;
    }

    // 清除现有图表
    if (tradingChart) {
        tradingChart.remove();
        tradingChart = null;
    }

    // 如果没有K线数据，显示提示
    if (!klineData || klineData.length === 0) {
        container.innerHTML = '<div style="display:flex;align-items:center;justify-content:center;height:100%;color:#64748b;">暂无K线数据</div>';
        console.warn('没有K线数据');
        return;
    }

    // 创建图表
    tradingChart = LightweightCharts.createChart(container, {
        width: container.clientWidth,
        height: container.clientHeight,
        layout: {
            background: { color: '#ffffff' },
            textColor: '#333',
        },
        grid: {
            vertLines: { color: '#f0f0f0' },
            horzLines: { color: '#f0f0f0' },
        },
        crosshair: {
            mode: LightweightCharts.CrosshairMode.Normal,
        },
        rightPriceScale: {
            borderColor: '#e0e0e0',
        },
        timeScale: {
            borderColor: '#e0e0e0',
            timeVisible: true,
            secondsVisible: false,
        },
    });

    // 创建K线系列
    candlestickSeries = tradingChart.addCandlestickSeries({
        upColor: '#10b981',
        downColor: '#ef4444',
        borderUpColor: '#10b981',
        borderDownColor: '#ef4444',
        wickUpColor: '#10b981',
        wickDownColor: '#ef4444',
    });

    // 转换K线数据格式
    const formattedKlines = klineData.map(k => ({
        time: Math.floor(k.timestamp / 1000), // 转换为秒级时间戳
        open: k.open,
        high: k.high,
        low: k.low,
        close: k.close,
    }));

    // 设置K线数据
    candlestickSeries.setData(formattedKlines);

    // 如果有交易数据，添加标记
    if (trades && trades.length > 0) {
        const markers = trades.map(trade => {
            const isBuy = trade.is_buy;
            return {
                time: Math.floor(trade.timestamp / 1000),
                position: isBuy ? 'belowBar' : 'aboveBar',
                color: isBuy ? '#10b981' : '#ef4444',
                shape: isBuy ? 'arrowUp' : 'arrowDown',
                text: isBuy ? `买入 @${trade.price.toFixed(2)}` : `卖出 @${trade.price.toFixed(2)}`,
                size: 1,
            };
        });

        candlestickSeries.setMarkers(markers);
        console.log('添加交易标记:', markers.length);
    }

    // 自适应时间范围
    tradingChart.timeScale().fitContent();

    // 响应式调整
    const resizeObserver = new ResizeObserver(entries => {
        if (tradingChart && entries.length > 0) {
            const { width, height } = entries[0].contentRect;
            tradingChart.applyOptions({ width, height });
        }
    });

    resizeObserver.observe(container);

    console.log('交易图表渲染完成，K线数:', klineData.length, '交易数:', trades ? trades.length : 0);
}

/**
 * 从CSV数据加载K线
 * @param {string} csvData - CSV格式的K线数据
 * @returns {Array} K线数组
 */
function parseKlineCSV(csvData) {
    const lines = csvData.trim().split('\n');
    const klines = [];

    // 跳过第一行表头
    for (let i = 1; i < lines.length; i++) {
        const parts = lines[i].split(',');
        if (parts.length >= 6) {
            klines.push({
                timestamp: parseInt(parts[0]),
                open: parseFloat(parts[1]),
                high: parseFloat(parts[2]),
                low: parseFloat(parts[3]),
                close: parseFloat(parts[4]),
                volume: parseFloat(parts[5]),
            });
        }
    }

    return klines;
}

/**
 * 从回测结果获取数据文件路径并加载K线数据
 * @param {string} dataPath - 数据文件路径
 * @param {Array} trades - 交易记录
 */
async function loadAndRenderTradingChart(dataPath, trades) {
    try {
        if (!dataPath) {
            console.warn('未提供数据路径');
            renderTradingChart([], trades);
            return;
        }

        console.log('尝试加载K线数据:', dataPath);
        
        // 调用新的 API 端点加载K线数据
        const response = await fetch(`/api/data/klines?path=${encodeURIComponent(dataPath)}`);
        
        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.message || '加载K线数据失败');
        }
        
        const result = await response.json();
        const klineData = result.data;
        
        console.log(`成功加载 ${klineData.length} 条K线数据`);
        
        // 渲染图表
        renderTradingChart(klineData, trades);
        
    } catch (error) {
        console.error('加载K线数据失败:', error);
        
        // 显示错误提示
        const container = document.getElementById('trading-chart');
        if (container) {
            container.innerHTML = `
                <div style="display:flex;flex-direction:column;align-items:center;justify-content:center;height:100%;color:#ef4444;gap:12px;">
                    <div style="font-size:48px;">⚠️</div>
                    <div>K线数据加载失败</div>
                    <div style="font-size:12px;color:#64748b;">数据路径: ${dataPath}</div>
                    <div style="font-size:12px;color:#64748b;">错误: ${error.message}</div>
                </div>
            `;
        }
        
        showNotification('K线数据加载失败: ' + error.message, 'error');
    }
}
