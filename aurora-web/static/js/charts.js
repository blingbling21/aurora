// Aurora Web - 图表可视化

let equityChart = null;

// 渲染权益曲线
function renderEquityCurve(data) {
    const canvas = document.getElementById('equity-chart');
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    
    // 销毁现有图表
    if (equityChart) {
        equityChart.destroy();
    }
    
    // 示例数据 - 实际应从回测结果中获取
    const labels = data.timestamps || generateSampleTimestamps(100);
    const equityData = data.equity || generateSampleEquity(100);
    
    equityChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                {
                    label: '权益曲线',
                    data: equityData,
                    borderColor: 'rgb(59, 130, 246)',
                    backgroundColor: 'rgba(59, 130, 246, 0.1)',
                    borderWidth: 2,
                    fill: true,
                    tension: 0.4
                }
            ]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: {
                    display: true,
                    position: 'top'
                },
                title: {
                    display: true,
                    text: '账户权益曲线',
                    font: {
                        size: 16
                    }
                },
                tooltip: {
                    mode: 'index',
                    intersect: false
                }
            },
            scales: {
                x: {
                    display: true,
                    title: {
                        display: true,
                        text: '时间'
                    },
                    ticks: {
                        maxTicksLimit: 10
                    }
                },
                y: {
                    display: true,
                    title: {
                        display: true,
                        text: '权益 (USD)'
                    },
                    ticks: {
                        callback: function(value) {
                            return '$' + value.toLocaleString();
                        }
                    }
                }
            },
            interaction: {
                mode: 'nearest',
                axis: 'x',
                intersect: false
            }
        }
    });
}

// 生成示例时间戳
function generateSampleTimestamps(count) {
    const timestamps = [];
    const now = Date.now();
    const dayMs = 24 * 60 * 60 * 1000;
    
    for (let i = 0; i < count; i++) {
        const date = new Date(now - (count - i) * dayMs);
        timestamps.push(date.toLocaleDateString('zh-CN'));
    }
    
    return timestamps;
}

// 生成示例权益数据
function generateSampleEquity(count) {
    const equity = [];
    let current = 10000;
    
    for (let i = 0; i < count; i++) {
        const change = (Math.random() - 0.48) * 200; // 略微上涨趋势
        current += change;
        equity.push(Math.round(current * 100) / 100);
    }
    
    return equity;
}

// 渲染回撤曲线
function renderDrawdownCurve(data) {
    // 类似实现...
}

// 渲染交易信号图
function renderTradeSignals(data) {
    // 类似实现...
}
