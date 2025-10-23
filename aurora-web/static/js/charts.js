// Aurora Web - 图表可视化

let equityChart = null;
let drawdownChart = null;

/**
 * 渲染权益曲线
 * @param {Array} equityCurve - 策略权益曲线数据 [{timestamp, equity, drawdown}]
 * @param {Array} benchmarkCurve - 基准权益曲线数据（可选）[{timestamp, equity, drawdown}]
 */
function renderEquityCurve(equityCurve, benchmarkCurve = null) {
    const canvas = document.getElementById('equity-chart');
    if (!canvas) {
        console.error('找不到权益曲线画布');
        return;
    }
    
    const ctx = canvas.getContext('2d');
    
    // 销毁现有图表
    if (equityChart) {
        equityChart.destroy();
    }
    
    // 如果没有数据，使用示例数据
    if (!equityCurve || equityCurve.length === 0) {
        console.warn('没有权益曲线数据，使用示例数据');
        const sampleTimestamps = generateSampleTimestamps(100);
        const sampleEquity = generateSampleEquity(100);
        equityCurve = sampleTimestamps.map((ts, i) => ({
            timestamp: new Date(ts).getTime(),
            equity: sampleEquity[i],
            drawdown: 0
        }));
    }
    
    // 准备数据
    const labels = equityCurve.map(point => {
        const date = new Date(point.timestamp);
        return date.toLocaleString('zh-CN', {
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit'
        });
    });
    
    const equityData = equityCurve.map(point => point.equity);
    
    // 计算策略初始权益和最终权益
    const initialEquity = equityCurve[0].equity;
    const finalEquity = equityCurve[equityCurve.length - 1].equity;
    const totalReturn = ((finalEquity - initialEquity) / initialEquity * 100).toFixed(2);
    
    // 准备数据集
    const datasets = [
        {
            label: `策略权益 (收益率: ${totalReturn}%)`,
            data: equityData,
            borderColor: totalReturn >= 0 ? 'rgb(34, 197, 94)' : 'rgb(239, 68, 68)',
            backgroundColor: totalReturn >= 0 ? 'rgba(34, 197, 94, 0.1)' : 'rgba(239, 68, 68, 0.1)',
            borderWidth: 2,
            fill: true,
            tension: 0.4,
            pointRadius: 0,
            pointHoverRadius: 5
        }
    ];
    
    // 如果有基准数据，添加基准曲线
    if (benchmarkCurve && benchmarkCurve.length > 0) {
        const benchmarkData = benchmarkCurve.map(point => point.equity);
        const benchmarkInitial = benchmarkCurve[0].equity;
        const benchmarkFinal = benchmarkCurve[benchmarkCurve.length - 1].equity;
        const benchmarkReturn = ((benchmarkFinal - benchmarkInitial) / benchmarkInitial * 100).toFixed(2);
        
        datasets.push({
            label: `基准 (Buy & Hold) (收益率: ${benchmarkReturn}%)`,
            data: benchmarkData,
            borderColor: 'rgb(59, 130, 246)',
            backgroundColor: 'rgba(59, 130, 246, 0.05)',
            borderWidth: 2,
            borderDash: [5, 5], // 虚线
            fill: false,
            tension: 0.4,
            pointRadius: 0,
            pointHoverRadius: 5
        });
    }
    
    equityChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: datasets
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
                        size: 16,
                        weight: 'bold'
                    }
                },
                tooltip: {
                    mode: 'index',
                    intersect: false,
                    callbacks: {
                        label: function(context) {
                            let label = context.dataset.label || '';
                            if (label) {
                                label += ': ';
                            }
                            label += '$' + context.parsed.y.toLocaleString('en-US', {
                                minimumFractionDigits: 2,
                                maximumFractionDigits: 2
                            });
                            return label;
                        }
                    }
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
                        maxTicksLimit: 10,
                        autoSkip: true
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
                            return '$' + value.toLocaleString('en-US');
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
    
    console.log('权益曲线渲染完成，数据点数:', equityCurve.length);
}

/**
 * 渲染回撤曲线
 * @param {Array} equityCurve - 权益曲线数据 [{timestamp, equity, drawdown}]
 */
function renderDrawdownCurve(equityCurve) {
    const canvas = document.getElementById('drawdown-chart');
    if (!canvas) {
        console.warn('找不到回撤曲线画布');
        return;
    }
    
    const ctx = canvas.getContext('2d');
    
    // 销毁现有图表
    if (drawdownChart) {
        drawdownChart.destroy();
    }
    
    if (!equityCurve || equityCurve.length === 0) {
        console.warn('没有回撤数据');
        return;
    }
    
    // 准备数据
    const labels = equityCurve.map(point => {
        const date = new Date(point.timestamp);
        return date.toLocaleString('zh-CN', {
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit'
        });
    });
    
    const drawdownData = equityCurve.map(point => -point.drawdown); // 负值显示为向下
    const maxDrawdown = Math.max(...equityCurve.map(p => p.drawdown));
    
    drawdownChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                {
                    label: `回撤 (最大: ${maxDrawdown.toFixed(2)}%)`,
                    data: drawdownData,
                    borderColor: 'rgb(239, 68, 68)',
                    backgroundColor: 'rgba(239, 68, 68, 0.2)',
                    borderWidth: 2,
                    fill: true,
                    tension: 0.4,
                    pointRadius: 0,
                    pointHoverRadius: 5
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
                    text: '回撤曲线',
                    font: {
                        size: 16,
                        weight: 'bold'
                    }
                },
                tooltip: {
                    mode: 'index',
                    intersect: false,
                    callbacks: {
                        label: function(context) {
                            return '回撤: ' + (-context.parsed.y).toFixed(2) + '%';
                        }
                    }
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
                        maxTicksLimit: 10,
                        autoSkip: true
                    }
                },
                y: {
                    display: true,
                    title: {
                        display: true,
                        text: '回撤 (%)'
                    },
                    ticks: {
                        callback: function(value) {
                            return (-value).toFixed(1) + '%';
                        }
                    },
                    reverse: false
                }
            },
            interaction: {
                mode: 'nearest',
                axis: 'x',
                intersect: false
            }
        }
    });
    
    console.log('回撤曲线渲染完成');
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

// 渲染交易信号图
function renderTradeSignals(data) {
    // 类似实现...
}

/**
 * 销毁所有图表
 */
function destroyAllCharts() {
    if (equityChart) {
        equityChart.destroy();
        equityChart = null;
    }
    if (drawdownChart) {
        drawdownChart.destroy();
        drawdownChart = null;
    }
}
