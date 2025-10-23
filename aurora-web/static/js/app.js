// Aurora Web - 主应用逻辑

const API_BASE = '/api';

// 全局状态
const appState = {
    currentPage: 'dashboard',
    configs: [],
    dataFiles: [],
    tasks: [],
    currentTaskId: null,
    wsConnection: null
};

// 初始化应用
document.addEventListener('DOMContentLoaded', () => {
    initNavigation();
    loadDashboard();
});

// 导航功能
function initNavigation() {
    const navItems = document.querySelectorAll('.nav-item');
    navItems.forEach(item => {
        item.addEventListener('click', () => {
            const page = item.dataset.page;
            navigateToPage(page);
        });
    });
}

function navigateToPage(pageName) {
    // 更新导航状态
    document.querySelectorAll('.nav-item').forEach(item => {
        item.classList.toggle('active', item.dataset.page === pageName);
    });

    // 显示对应页面
    document.querySelectorAll('.page').forEach(page => {
        page.classList.toggle('active', page.id === `${pageName}-page`);
    });

    appState.currentPage = pageName;

    // 加载页面数据
    switch (pageName) {
        case 'dashboard':
            loadDashboard();
            break;
        case 'config':
            loadConfigs();
            break;
        case 'data':
            loadDataFiles();
            break;
        case 'backtest':
            loadBacktestPage();
            break;
        case 'history':
            loadHistory();
            break;
    }
}

// 加载仪表盘
async function loadDashboard() {
    try {
        const response = await fetch(`${API_BASE}/backtest/history`);
        const result = await response.json();
        
        if (result.success) {
            appState.tasks = result.data;
            updateDashboardStats();
            displayRecentTasks();
        }
    } catch (error) {
        showNotification('加载仪表盘数据失败', 'error');
        console.error(error);
    }
}

function updateDashboardStats() {
    const total = appState.tasks.length;
    const running = appState.tasks.filter(t => t.status === 'running').length;
    const completed = appState.tasks.filter(t => t.status === 'completed').length;
    const failed = appState.tasks.filter(t => t.status === 'failed').length;

    document.getElementById('total-tasks').textContent = total;
    document.getElementById('running-tasks').textContent = running;
    document.getElementById('completed-tasks').textContent = completed;
    document.getElementById('failed-tasks').textContent = failed;
}

function displayRecentTasks() {
    const container = document.getElementById('recent-tasks-list');
    const recentTasks = appState.tasks.slice(0, 5);

    if (recentTasks.length === 0) {
        container.innerHTML = '<p style="color: var(--text-secondary);">暂无任务记录</p>';
        return;
    }

    container.innerHTML = recentTasks.map(task => `
        <div class="task-item" onclick="viewTaskResult('${task.id}')" style="cursor: pointer;">
            <div class="task-header">
                <span class="task-name">${task.name}</span>
                <span class="task-status status-${task.status}">${getStatusText(task.status)}</span>
            </div>
            <div class="task-meta">
                <span>📅 ${formatDate(task.created_at)}</span>
                <span>⏱️ 进度: ${task.progress}%</span>
            </div>
            ${task.status === 'completed' ? `<div style="color: var(--primary-color); font-size: 11px; margin-top: 6px;">💡 点击查看详细结果</div>` : ''}
        </div>
    `).join('');
}

// 工具函数
function getStatusText(status) {
    const statusMap = {
        'pending': '等待中',
        'running': '运行中',
        'completed': '已完成',
        'failed': '失败'
    };
    return statusMap[status] || status;
}

function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleString('zh-CN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit'
    });
}

function formatFileSize(bytes) {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB';
    return (bytes / (1024 * 1024)).toFixed(2) + ' MB';
}

// 通知系统
function showNotification(message, type = 'info') {
    const notification = document.getElementById('notification');
    notification.textContent = message;
    notification.className = `notification ${type} show`;

    setTimeout(() => {
        notification.classList.remove('show');
    }, 3000);
}

// API请求封装
async function apiRequest(endpoint, options = {}) {
    const url = `${API_BASE}${endpoint}`;
    const config = {
        ...options,
        headers: {
            'Content-Type': 'application/json',
            ...options.headers
        }
    };

    try {
        const response = await fetch(url, config);
        const data = await response.json();
        
        if (!response.ok) {
            throw new Error(data.message || '请求失败');
        }
        
        return data;
    } catch (error) {
        console.error('API请求错误:', error);
        throw error;
    }
}

// 查看任务结果
async function viewTaskResult(taskId) {
    try {
        const response = await apiRequest(`/backtest/result/${taskId}`);
        
        if (response.success && response.data) {
            // 确保在历史记录页面
            if (appState.currentPage !== 'history') {
                navigateToPage('history');
            }
            
            // 延迟一下确保页面已切换
            setTimeout(() => {
                displayTaskResult(taskId, response.data);
            }, 100);
        }
    } catch (error) {
        console.error('获取任务结果失败:', error);
        showNotification('任务尚未完成或执行失败', 'error');
    }
}

function displayTaskResult(taskId, resultData) {
    const viewer = document.getElementById('result-viewer');
    if (!viewer) {
        console.error('找不到 result-viewer 元素');
        return;
    }
    
    viewer.style.display = 'block';
    
    // 提取结果数据
    const result = resultData.result || resultData;
    const metrics = result.metrics || {};
    const equityCurve = result.equity_curve || [];
    const trades = result.trades || [];
    
    // 显示结果摘要 - 展示所有关键指标
    const summary = document.getElementById('result-summary');
    summary.innerHTML = `
        <!-- 收益指标 -->
        <div class="result-metric">
            <div class="result-metric-label">总收益率</div>
            <div class="result-metric-value ${metrics.total_return >= 0 ? 'positive' : 'negative'}">
                ${metrics.total_return ? metrics.total_return.toFixed(2) : '0.00'}%
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">年化收益率</div>
            <div class="result-metric-value ${metrics.annualized_return >= 0 ? 'positive' : 'negative'}">
                ${metrics.annualized_return ? metrics.annualized_return.toFixed(2) : '0.00'}%
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">最大回撤</div>
            <div class="result-metric-value negative">
                ${metrics.max_drawdown ? metrics.max_drawdown.toFixed(2) : '0.00'}%
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">回撤持续时间</div>
            <div class="result-metric-value">
                ${metrics.max_drawdown_duration ? metrics.max_drawdown_duration.toFixed(1) : '0.0'} 天
            </div>
        </div>
        
        <!-- 风险调整收益 -->
        <div class="result-metric">
            <div class="result-metric-label">夏普比率</div>
            <div class="result-metric-value">
                ${metrics.sharpe_ratio ? metrics.sharpe_ratio.toFixed(3) : '0.000'}
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">索提诺比率</div>
            <div class="result-metric-value">
                ${metrics.sortino_ratio ? metrics.sortino_ratio.toFixed(3) : '0.000'}
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">卡玛比率</div>
            <div class="result-metric-value">
                ${metrics.calmar_ratio ? metrics.calmar_ratio.toFixed(3) : '0.000'}
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">年化波动率</div>
            <div class="result-metric-value">
                ${metrics.annualized_volatility ? metrics.annualized_volatility.toFixed(2) : '0.00'}%
            </div>
        </div>
        ${result.alpha !== undefined && result.alpha !== null ? `
        <div class="result-metric">
            <div class="result-metric-label">Alpha (相对收益)</div>
            <div class="result-metric-value ${result.alpha >= 0 ? 'positive' : 'negative'}">
                ${result.alpha.toFixed(2)}%
            </div>
        </div>
        ` : ''}
        ${result.annualized_alpha !== undefined && result.annualized_alpha !== null ? `
        <div class="result-metric">
            <div class="result-metric-label">年化 Alpha</div>
            <div class="result-metric-value ${result.annualized_alpha >= 0 ? 'positive' : 'negative'}">
                ${result.annualized_alpha.toFixed(2)}%
            </div>
        </div>
        ` : ''}
        
        <!-- 交易统计 -->
        <div class="result-metric">
            <div class="result-metric-label">总交易次数</div>
            <div class="result-metric-value">
                ${metrics.total_trades || 0}
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">胜率</div>
            <div class="result-metric-value">
                ${metrics.win_rate ? metrics.win_rate.toFixed(2) : '0.00'}%
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">盈亏比</div>
            <div class="result-metric-value">
                ${metrics.profit_loss_ratio ? metrics.profit_loss_ratio.toFixed(2) : '0.00'}
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">利润因子</div>
            <div class="result-metric-value">
                ${metrics.profit_factor ? metrics.profit_factor.toFixed(2) : '0.00'}
            </div>
        </div>
        
        <!-- 盈亏分析 -->
        <div class="result-metric">
            <div class="result-metric-label">平均盈利</div>
            <div class="result-metric-value positive">
                $${metrics.average_win ? metrics.average_win.toFixed(2) : '0.00'}
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">平均亏损</div>
            <div class="result-metric-value negative">
                $${metrics.average_loss ? metrics.average_loss.toFixed(2) : '0.00'}
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">最大单笔盈利</div>
            <div class="result-metric-value positive">
                $${metrics.max_win ? metrics.max_win.toFixed(2) : '0.00'}
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">最大单笔亏损</div>
            <div class="result-metric-value negative">
                $${metrics.max_loss ? metrics.max_loss.toFixed(2) : '0.00'}
            </div>
        </div>
        
        <!-- 交易行为 -->
        <div class="result-metric">
            <div class="result-metric-label">最大连续盈利</div>
            <div class="result-metric-value">
                ${metrics.max_consecutive_wins || 0} 次
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">最大连续亏损</div>
            <div class="result-metric-value">
                ${metrics.max_consecutive_losses || 0} 次
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">平均持仓时间</div>
            <div class="result-metric-value">
                ${metrics.avg_holding_period ? metrics.avg_holding_period.toFixed(2) : '0.00'} 小时
            </div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">交易记录</div>
            <div class="result-metric-value">
                ${trades.length} 条
            </div>
        </div>
    `;
    
    // 渲染图表
    if (equityCurve && equityCurve.length > 0) {
        console.log('渲染权益曲线，数据点:', equityCurve.length);
        
        // 获取基准数据（如果有）
        const benchmarkCurve = resultData.benchmark_equity_curve || null;
        if (benchmarkCurve) {
            console.log('找到基准数据，数据点:', benchmarkCurve.length);
        }
        
        renderEquityCurve(equityCurve, benchmarkCurve);
        renderDrawdownCurve(equityCurve);
        
        // 渲染交易点位图（如果有交易数据）
        if (trades && trades.length > 0) {
            // 假设数据路径存储在结果中（需要在后端添加）
            const dataPath = result.data_path || '数据路径未知';
            loadAndRenderTradingChart(dataPath, trades);
        }
    } else {
        console.warn('没有权益曲线数据');
        showNotification('没有图表数据', 'warning');
    }
    
    // 滚动到结果查看器
    viewer.scrollIntoView({ behavior: 'smooth', block: 'start' });
}
