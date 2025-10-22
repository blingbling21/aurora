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
        <div class="task-item" onclick="viewTaskResult('${task.id}')">
            <div class="task-header">
                <span class="task-name">${task.name}</span>
                <span class="task-status status-${task.status}">${getStatusText(task.status)}</span>
            </div>
            <div class="task-meta">
                <span>📅 ${formatDate(task.created_at)}</span>
                <span>⏱️ 进度: ${task.progress}%</span>
            </div>
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
            navigateToPage('history');
            displayTaskResult(taskId, response.data);
        }
    } catch (error) {
        showNotification('任务尚未完成或执行失败', 'error');
    }
}

function displayTaskResult(taskId, result) {
    const viewer = document.getElementById('result-viewer');
    viewer.style.display = 'block';
    
    // 显示结果摘要
    const summary = document.getElementById('result-summary');
    summary.innerHTML = `
        <div class="result-metric">
            <div class="result-metric-label">任务ID</div>
            <div class="result-metric-value" style="font-size: 16px;">${taskId}</div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">状态</div>
            <div class="result-metric-value" style="font-size: 16px;">${result.status || '已完成'}</div>
        </div>
        <div class="result-metric">
            <div class="result-metric-label">策略类型</div>
            <div class="result-metric-value" style="font-size: 16px;">${result.strategy || 'N/A'}</div>
        </div>
    `;
    
    // 如果有更多指标数据，可以在这里添加图表展示
    // renderEquityCurve(result);
}
