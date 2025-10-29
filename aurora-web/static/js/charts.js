// Aurora Web - 图表可视化（增强版）

let equityChart = null;
let drawdownChart = null;
let yAxisScaleMode = 'adaptive'; // 'adaptive', 'fixed', 'logarithmic'

/**
 * 渲染权益曲线（增强版）
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
            pointHoverRadius: 5,
            yAxisID: 'y'
        }
    ];
    
    // 如果有基准数据，添加基准曲线
    let alphaReturn = null;
    if (benchmarkCurve && benchmarkCurve.length > 0) {
        const benchmarkData = benchmarkCurve.map(point => point.equity);
        const benchmarkInitial = benchmarkCurve[0].equity;
        const benchmarkFinal = benchmarkCurve[benchmarkCurve.length - 1].equity;
        const benchmarkReturn = ((benchmarkFinal - benchmarkInitial) / benchmarkInitial * 100).toFixed(2);
        
        // 计算超额收益（Alpha）
        alphaReturn = (totalReturn - benchmarkReturn).toFixed(2);
        
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
            pointHoverRadius: 5,
            yAxisID: 'y'
        });
    }
    
    // 计算Y轴范围
    let yAxisOptions = getYAxisOptions(equityData, benchmarkCurve);
    
    // 创建标题文本（包含Alpha信息）
    let titleText = '账户权益曲线';
    if (alphaReturn !== null) {
        const alphaSign = alphaReturn >= 0 ? '+' : '';
        titleText += ` | 超额收益 (Alpha): ${alphaSign}${alphaReturn}%`;
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
                    position: 'top',
                    labels: {
                        usePointStyle: true,
                        padding: 15,
                        font: {
                            size: 12
                        }
                    }
                },
                title: {
                    display: true,
                    text: titleText,
                    font: {
                        size: 16,
                        weight: 'bold'
                    },
                    padding: {
                        bottom: 20
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
                        },
                        afterBody: function(tooltipItems) {
                            // 添加时间点的回撤信息
                            const index = tooltipItems[0].dataIndex;
                            if (equityCurve[index] && equityCurve[index].drawdown) {
                                return `回撤: ${equityCurve[index].drawdown.toFixed(2)}%`;
                            }
                            return '';
                        }
                    }
                }
            },
            scales: {
                x: {
                    display: true,
                    title: {
                        display: true,
                        text: '时间',
                        font: {
                            size: 13,
                            weight: 'bold'
                        }
                    },
                    ticks: {
                        maxTicksLimit: 10,
                        autoSkip: true
                    }
                },
                y: yAxisOptions
            },
            interaction: {
                mode: 'nearest',
                axis: 'x',
                intersect: false
            }
        }
    });
    
    // 保存到全局变量供联动使用
    window.equityChart = equityChart;
    
    // 添加Y轴切换按钮（如果还没有）
    addYAxisToggleButton();
    
    console.log('权益曲线渲染完成，数据点数:', equityCurve.length, 'Y轴模式:', yAxisScaleMode);
}

/**
 * 根据当前Y轴模式获取Y轴配置
 */
function getYAxisOptions(equityData, benchmarkCurve) {
    let allData = [...equityData];
    if (benchmarkCurve) {
        allData = allData.concat(benchmarkCurve.map(p => p.equity));
    }
    
    const minValue = Math.min(...allData);
    const maxValue = Math.max(...allData);
    const initialValue = equityData[0];
    
    const baseOptions = {
        display: true,
        title: {
            display: true,
            text: '权益 (USD)',
            font: {
                size: 13,
                weight: 'bold'
            }
        },
        ticks: {
            callback: function(value) {
                return '$' + value.toLocaleString('en-US');
            }
        }
    };
    
    switch (yAxisScaleMode) {
        case 'fixed':
            // 固定范围：初始资金的80%-120%
            return {
                ...baseOptions,
                min: initialValue * 0.8,
                max: initialValue * 1.2,
                ticks: {
                    ...baseOptions.ticks,
                    stepSize: initialValue * 0.1
                }
            };
        
        case 'logarithmic':
            // 对数坐标轴
            return {
                ...baseOptions,
                type: 'logarithmic',
                ticks: {
                    ...baseOptions.ticks,
                    callback: function(value) {
                        // 只显示整数刻度
                        if (value === Math.floor(value)) {
                            return '$' + value.toLocaleString('en-US');
                        }
                        return '';
                    }
                }
            };
        
        case 'adaptive':
        default:
            // 自适应范围（默认）
            const padding = (maxValue - minValue) * 0.05;
            return {
                ...baseOptions,
                min: Math.floor(minValue - padding),
                max: Math.ceil(maxValue + padding)
            };
    }
}

/**
 * 添加Y轴切换按钮
 */
function addYAxisToggleButton() {
    // 检查是否已存在
    if (document.getElementById('y-axis-toggle')) {
        return;
    }
    
    const canvas = document.getElementById('equity-chart');
    const container = canvas.parentElement;
    
    const buttonGroup = document.createElement('div');
    buttonGroup.id = 'y-axis-toggle';
    buttonGroup.style.cssText = `
        position: absolute;
        top: 10px;
        right: 10px;
        display: flex;
        gap: 5px;
        z-index: 10;
    `;
    
    const modes = [
        { value: 'adaptive', label: '自适应' },
        { value: 'fixed', label: '固定范围' },
        { value: 'logarithmic', label: '对数' }
    ];
    
    modes.forEach(mode => {
        const btn = document.createElement('button');
        btn.textContent = mode.label;
        btn.className = yAxisScaleMode === mode.value ? 'axis-btn active' : 'axis-btn';
        btn.style.cssText = `
            padding: 4px 10px;
            font-size: 11px;
            border: 1px solid #cbd5e1;
            background: ${yAxisScaleMode === mode.value ? '#3b82f6' : '#ffffff'};
            color: ${yAxisScaleMode === mode.value ? '#ffffff' : '#475569'};
            border-radius: 4px;
            cursor: pointer;
            transition: all 0.2s;
        `;
        
        btn.addEventListener('click', () => {
            yAxisScaleMode = mode.value;
            // 重新渲染图表
            const equityCurve = window.currentEquityCurve;
            const benchmarkCurve = window.currentBenchmarkCurve;
            if (equityCurve) {
                renderEquityCurve(equityCurve, benchmarkCurve);
                
                // 重要：重新注册到联动控制器
                if (typeof registerChartToSync === 'function' && window.equityChart) {
                    registerChartToSync('equity', window.equityChart);
                }
            }
        });
        
        btn.addEventListener('mouseenter', () => {
            if (yAxisScaleMode !== mode.value) {
                btn.style.background = '#f1f5f9';
            }
        });
        
        btn.addEventListener('mouseleave', () => {
            if (yAxisScaleMode !== mode.value) {
                btn.style.background = '#ffffff';
            }
        });
        
        buttonGroup.appendChild(btn);
    });
    
    container.style.position = 'relative';
    container.appendChild(buttonGroup);
}

/**
 * 渲染回撤曲线（增强版）
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
    
    // 找到最大回撤的区间
    const maxDDInfo = findMaxDrawdownPeriod(equityCurve);
    
    // 创建背景颜色数组（高亮最大回撤区间）
    const backgroundColors = drawdownData.map((_, index) => {
        if (maxDDInfo && index >= maxDDInfo.startIndex && index <= maxDDInfo.endIndex) {
            return 'rgba(239, 68, 68, 0.35)'; // 高亮区域
        }
        return 'rgba(239, 68, 68, 0.15)';
    });
    
    drawdownChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                {
                    label: `回撤 (最大: ${maxDrawdown.toFixed(2)}%)`,
                    data: drawdownData,
                    borderColor: 'rgb(239, 68, 68)',
                    backgroundColor: backgroundColors,
                    borderWidth: 2,
                    fill: true,
                    tension: 0.4,
                    pointRadius: 0,
                    pointHoverRadius: 5,
                    segment: {
                        backgroundColor: ctx => {
                            const index = ctx.p0DataIndex;
                            return backgroundColors[index];
                        }
                    }
                }
            ]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: {
                    display: true,
                    position: 'top',
                    labels: {
                        usePointStyle: true,
                        padding: 15,
                        font: {
                            size: 12
                        }
                    }
                },
                title: {
                    display: true,
                    text: '回撤曲线',
                    font: {
                        size: 16,
                        weight: 'bold'
                    },
                    padding: {
                        bottom: 20
                    }
                },
                tooltip: {
                    mode: 'index',
                    intersect: false,
                    callbacks: {
                        label: function(context) {
                            const index = context.dataIndex;
                            let label = '回撤: ' + (-context.parsed.y).toFixed(2) + '%';
                            
                            // 如果在最大回撤区间内，添加特殊标记
                            if (maxDDInfo && index >= maxDDInfo.startIndex && index <= maxDDInfo.endIndex) {
                                if (index === maxDDInfo.maxIndex) {
                                    label += ' ⚠️ 最大回撤点';
                                } else {
                                    label += ' (最大回撤区间内)';
                                }
                            }
                            
                            return label;
                        },
                        afterBody: function(tooltipItems) {
                            const index = tooltipItems[0].dataIndex;
                            
                            // 如果是最大回撤点，显示详细信息
                            if (maxDDInfo && index === maxDDInfo.maxIndex) {
                                return [
                                    '',
                                    `📊 最大回撤详情:`,
                                    `   幅度: ${maxDDInfo.maxDD.toFixed(2)}%`,
                                    `   开始: ${maxDDInfo.startTime}`,
                                    `   最低点: ${maxDDInfo.maxTime}`,
                                    `   恢复: ${maxDDInfo.endTime || '未恢复'}`,
                                    `   持续: ${maxDDInfo.duration.toFixed(1)} 天`
                                ];
                            }
                            
                            return '';
                        }
                    }
                },
                annotation: maxDDInfo ? {
                    annotations: {
                        maxDDBox: {
                            type: 'box',
                            xMin: maxDDInfo.startIndex,
                            xMax: maxDDInfo.endIndex,
                            backgroundColor: 'rgba(239, 68, 68, 0.08)',
                            borderColor: 'rgba(239, 68, 68, 0.5)',
                            borderWidth: 1,
                            borderDash: [5, 5],
                            label: {
                                display: true,
                                content: `最大回撤区间 (${maxDDInfo.duration.toFixed(0)}天)`,
                                position: 'start',
                                font: {
                                    size: 10
                                },
                                color: '#ef4444'
                            }
                        }
                    }
                } : {}
            },
            scales: {
                x: {
                    display: true,
                    title: {
                        display: true,
                        text: '时间',
                        font: {
                            size: 13,
                            weight: 'bold'
                        }
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
                        text: '回撤 (%)',
                        font: {
                            size: 13,
                            weight: 'bold'
                        }
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
    
    // 保存到全局变量供联动使用
    window.drawdownChart = drawdownChart;
    
    console.log('回撤曲线渲染完成', maxDDInfo ? `，最大回撤区间: ${maxDDInfo.startIndex}-${maxDDInfo.endIndex}` : '');
}

/**
 * 找到最大回撤期间
 * @param {Array} equityCurve - 权益曲线数据
 * @returns {Object} 最大回撤信息
 */
function findMaxDrawdownPeriod(equityCurve) {
    if (!equityCurve || equityCurve.length === 0) {
        return null;
    }
    
    let maxDD = 0;
    let maxDDIndex = -1;
    let startIndex = -1;
    let endIndex = -1;
    let peakIndex = 0;
    let currentPeak = equityCurve[0].equity;
    
    // 找到最大回撤点和起始点
    for (let i = 0; i < equityCurve.length; i++) {
        const equity = equityCurve[i].equity;
        const drawdown = equityCurve[i].drawdown;
        
        if (equity > currentPeak) {
            currentPeak = equity;
            peakIndex = i;
        }
        
        if (drawdown > maxDD) {
            maxDD = drawdown;
            maxDDIndex = i;
            startIndex = peakIndex;
        }
    }
    
    // 找到恢复点（如果有）
    if (maxDDIndex >= 0 && maxDDIndex < equityCurve.length - 1) {
        const maxDDEquity = equityCurve[maxDDIndex].equity;
        const targetEquity = equityCurve[startIndex].equity;
        
        for (let i = maxDDIndex + 1; i < equityCurve.length; i++) {
            if (equityCurve[i].equity >= targetEquity) {
                endIndex = i;
                break;
            }
        }
    }
    
    // 如果没有恢复，结束点就是最后一个点
    if (endIndex === -1) {
        endIndex = equityCurve.length - 1;
    }
    
    // 计算持续时间（天）
    const startTime = new Date(equityCurve[startIndex].timestamp);
    const endTime = new Date(equityCurve[endIndex].timestamp);
    const duration = (endTime - startTime) / (1000 * 60 * 60 * 24);
    
    return {
        maxDD: maxDD,
        maxIndex: maxDDIndex,
        startIndex: startIndex,
        endIndex: endIndex,
        startTime: startTime.toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' }),
        maxTime: new Date(equityCurve[maxDDIndex].timestamp).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' }),
        endTime: endIndex < equityCurve.length - 1 ? endTime.toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' }) : null,
        duration: duration
    };
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
