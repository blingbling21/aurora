// Aurora Web - 图表联动功能（Chart Synchronization）

/**
 * 图表同步控制器
 * 负责同步多个图表的光标位置、缩放和平移
 */
class ChartSyncController {
    constructor() {
        this.charts = {
            equity: null,
            drawdown: null,
            trading: null
        };
        
        this.isHovering = false;
        this.currentIndex = null;
        this.syncEnabled = true;
    }
    
    /**
     * 注册图表实例
     */
    registerCharts(equityChart, drawdownChart, tradingChart) {
        this.charts.equity = equityChart;
        this.charts.drawdown = drawdownChart;
        this.charts.trading = tradingChart;
        
        // 为Chart.js图表添加交互事件
        if (equityChart) {
            this.attachChartJsEvents(equityChart, 'equity');
        }
        if (drawdownChart) {
            this.attachChartJsEvents(drawdownChart, 'drawdown');
        }
        
        // 为Lightweight Charts添加交互事件
        if (tradingChart) {
            this.attachLightweightChartsEvents(tradingChart);
        }
        
        console.log('图表联动已启用');
    }
    
    /**
     * 为Chart.js图表附加事件
     */
    attachChartJsEvents(chart, chartType) {
        const canvas = chart.canvas;
        
        // 鼠标移动事件
        canvas.addEventListener('mousemove', (event) => {
            if (!this.syncEnabled) return;
            
            const rect = canvas.getBoundingClientRect();
            const x = event.clientX - rect.left;
            const y = event.clientY - rect.top;
            
            // 获取数据点索引
            const activePoints = chart.getElementsAtEventForMode(
                event,
                'index',
                { intersect: false },
                false
            );
            
            if (activePoints.length > 0) {
                const index = activePoints[0].index;
                this.syncCrosshair(chartType, index);
            }
        });
        
        // 鼠标离开事件
        canvas.addEventListener('mouseleave', () => {
            this.clearCrosshair(chartType);
        });
    }
    
    /**
     * 为Lightweight Charts附加事件
     */
    attachLightweightChartsEvents(chart) {
        chart.subscribeCrosshairMove((param) => {
            if (!this.syncEnabled) return;
            
            if (param.time) {
                // 转换时间戳到索引（需要从数据中映射）
                const timestamp = param.time * 1000; // 转换为毫秒
                this.syncCrosshairByTimestamp('trading', timestamp);
            } else {
                this.clearCrosshair('trading');
            }
        });
    }
    
    /**
     * 同步十字光标位置
     */
    syncCrosshair(sourceChart, index) {
        this.currentIndex = index;
        
        // 更新其他Chart.js图表
        Object.keys(this.charts).forEach(chartType => {
            if (chartType === sourceChart || chartType === 'trading') return;
            
            const chart = this.charts[chartType];
            if (!chart) return;
            
            // 触发工具提示显示
            this.showTooltipAtIndex(chart, index);
        });
        
        // 同步到Lightweight Charts
        if (sourceChart !== 'trading' && this.charts.trading) {
            this.syncToLightweightCharts(index);
        }
    }
    
    /**
     * 根据时间戳同步
     */
    syncCrosshairByTimestamp(sourceChart, timestamp) {
        // 从权益曲线数据中找到对应的索引
        if (window.currentEquityCurve) {
            const index = window.currentEquityCurve.findIndex(
                point => Math.abs(point.timestamp - timestamp) < 60000 // 1分钟误差
            );
            
            if (index >= 0) {
                this.syncCrosshair(sourceChart, index);
            }
        }
    }
    
    /**
     * 在指定索引显示工具提示
     */
    showTooltipAtIndex(chart, index) {
        if (!chart || !chart.data || !chart.data.labels || index < 0 || index >= chart.data.labels.length) {
            return;
        }
        
        try {
            // 获取第一个数据集的元素
            const meta = chart.getDatasetMeta(0);
            if (!meta || !meta.data || !meta.data[index]) {
                return;
            }
            
            const element = meta.data[index];
            
            // 设置活动元素
            chart.setActiveElements([
                { datasetIndex: 0, index: index }
            ]);
            
            // 更新工具提示位置
            chart.tooltip.setActiveElements([
                { datasetIndex: 0, index: index }
            ], {
                x: element.x,
                y: element.y
            });
            
            // 更新图表（不触发动画）
            chart.update('none');
        } catch (error) {
            console.error('显示工具提示失败:', error);
        }
    }
    
    /**
     * 同步到Lightweight Charts
     */
    syncToLightweightCharts(index) {
        if (!this.charts.trading || !window.currentEquityCurve) {
            return;
        }
        
        // 获取时间戳
        const timestamp = window.currentEquityCurve[index].timestamp;
        const timeInSeconds = Math.floor(timestamp / 1000);
        
        // 设置Lightweight Charts的十字光标位置
        // 注意：Lightweight Charts没有直接的API来设置十字光标位置
        // 我们可以通过移动到该时间点来实现
        this.charts.trading.timeScale().scrollToPosition(0, true);
    }
    
    /**
     * 清除十字光标
     */
    clearCrosshair(sourceChart) {
        this.currentIndex = null;
        
        // 清除Chart.js图表的活动元素
        Object.keys(this.charts).forEach(chartType => {
            if (chartType === 'trading') return;
            
            const chart = this.charts[chartType];
            if (chart) {
                chart.setActiveElements([]);
                chart.tooltip.setActiveElements([]);
                chart.update('none');
            }
        });
    }
    
    /**
     * 启用/禁用同步
     */
    toggleSync(enabled) {
        this.syncEnabled = enabled;
        console.log('图表联动', enabled ? '已启用' : '已禁用');
    }
    
    /**
     * 同步缩放范围
     */
    syncZoom(sourceChart, xMin, xMax) {
        if (!this.syncEnabled) return;
        
        // 同步Chart.js图表的缩放
        Object.keys(this.charts).forEach(chartType => {
            if (chartType === sourceChart || chartType === 'trading') return;
            
            const chart = this.charts[chartType];
            if (chart && chart.options.scales && chart.options.scales.x) {
                chart.options.scales.x.min = xMin;
                chart.options.scales.x.max = xMax;
                chart.update('none');
            }
        });
        
        // TODO: 同步Lightweight Charts的缩放
    }
    
    /**
     * 重置所有图表
     */
    reset() {
        Object.keys(this.charts).forEach(chartType => {
            const chart = this.charts[chartType];
            if (chart && chartType !== 'trading') {
                chart.resetZoom();
            } else if (chart && chartType === 'trading') {
                chart.timeScale().fitContent();
            }
        });
    }
}

// 创建全局实例
const chartSync = new ChartSyncController();

/**
 * 初始化图表联动
 * 在所有图表渲染完成后调用
 */
function initChartSync() {
    console.log('初始化图表联动...');
    
    // 等待图表实例创建
    setTimeout(() => {
        const equity = window.equityChart;
        const drawdown = window.drawdownChart;
        const trading = window.tradingChart;
        
        if (equity && drawdown && trading) {
            chartSync.registerCharts(equity, drawdown, trading);
            console.log('所有图表已注册到联动控制器');
        } else {
            console.warn('部分图表未就绪，联动功能可能不完整');
            if (equity && drawdown) {
                chartSync.registerCharts(equity, drawdown, null);
                console.log('Chart.js图表已注册到联动控制器');
            }
        }
    }, 500);
}

/**
 * 注册单个图表到联动控制器
 * 用于图表重新渲染后的重新注册
 */
function registerChartToSync(chartType, chartInstance) {
    if (!chartInstance) {
        console.warn(`无法注册${chartType}图表: 实例为空`);
        return;
    }
    
    // 更新控制器中的图表引用
    chartSync.charts[chartType] = chartInstance;
    
    // 重新附加事件监听器
    if (chartType === 'equity' || chartType === 'drawdown') {
        chartSync.attachChartJsEvents(chartInstance, chartType);
    } else if (chartType === 'trading') {
        chartSync.attachLightweightChartsEvents(chartInstance);
    }
    
    console.log(`${chartType}图表已重新注册到联动控制器`);
}

// 导出到全局
window.registerChartToSync = registerChartToSync;

/**
 * 添加联动控制按钮
 */
function addChartSyncToggle() {
    const resultViewer = document.getElementById('result-viewer');
    if (!resultViewer || document.getElementById('chart-sync-toggle')) {
        return;
    }
    
    const toggleContainer = document.createElement('div');
    toggleContainer.id = 'chart-sync-toggle';
    toggleContainer.style.cssText = `
        position: absolute;
        top: 20px;
        right: 20px;
        z-index: 100;
    `;
    
    const toggleBtn = document.createElement('button');
    toggleBtn.textContent = '🔗 图表联动';
    toggleBtn.className = 'btn btn-sm';
    toggleBtn.style.cssText = `
        padding: 6px 12px;
        font-size: 12px;
        background: #3b82f6;
        color: white;
        border: none;
        border-radius: 6px;
        cursor: pointer;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    `;
    
    let syncEnabled = true;
    
    toggleBtn.addEventListener('click', () => {
        syncEnabled = !syncEnabled;
        chartSync.toggleSync(syncEnabled);
        
        toggleBtn.textContent = syncEnabled ? '🔗 图表联动' : '🔓 图表联动';
        toggleBtn.style.background = syncEnabled ? '#3b82f6' : '#94a3b8';
    });
    
    toggleContainer.appendChild(toggleBtn);
    
    const chartsLayout = document.querySelector('.charts-layout');
    if (chartsLayout) {
        chartsLayout.style.position = 'relative';
        chartsLayout.appendChild(toggleContainer);
    }
}
