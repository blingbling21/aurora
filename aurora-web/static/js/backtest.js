// Aurora Web - 回测执行

let currentWs = null;

// 初始化回测页面
document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('backtest-form');
    if (form) {
        form.addEventListener('submit', handleBacktestSubmit);
    }
    
    const viewResultBtn = document.getElementById('view-result-btn');
    if (viewResultBtn) {
        viewResultBtn.addEventListener('click', () => {
            if (appState.currentTaskId) {
                viewTaskResult(appState.currentTaskId);
            }
        });
    }
});

// 加载回测页面
async function loadBacktestPage() {
    // 加载配置和数据文件选项
    await Promise.all([
        loadConfigOptions(),
        loadDataOptions()
    ]);
}

async function loadConfigOptions() {
    try {
        const response = await apiRequest('/config');
        if (response.success) {
            const select = document.getElementById('backtest-config');
            select.innerHTML = '<option value="">-- 请选择 --</option>' +
                response.data.map(config => 
                    `<option value="${config.filename}">${config.filename}</option>`
                ).join('');
        }
    } catch (error) {
        console.error('加载配置选项失败:', error);
    }
}

async function loadDataOptions() {
    try {
        const response = await apiRequest('/data/list');
        if (response.success) {
            const select = document.getElementById('backtest-data');
            select.innerHTML = '<option value="">-- 请选择 --</option>' +
                response.data.map(file => 
                    `<option value="${file.filename}">${file.filename} (${formatFileSize(file.size)})</option>`
                ).join('');
        }
    } catch (error) {
        console.error('加载数据选项失败:', error);
    }
}

// 处理回测提交
async function handleBacktestSubmit(event) {
    event.preventDefault();
    
    const name = document.getElementById('backtest-name').value;
    const configPath = document.getElementById('backtest-config').value;
    const dataPath = document.getElementById('backtest-data').value;
    
    if (!name || !configPath || !dataPath) {
        showNotification('请填写所有必填字段', 'error');
        return;
    }
    
    try {
        const response = await apiRequest('/backtest/start', {
            method: 'POST',
            body: JSON.stringify({
                name,
                config_path: configPath,
                data_path: dataPath
            })
        });
        
        if (response.success) {
            const taskId = response.data.task_id;
            appState.currentTaskId = taskId;
            
            showNotification('回测任务已启动', 'success');
            showProgressCard(name, taskId);
            connectWebSocket(taskId);
            
            // 清空表单
            document.getElementById('backtest-form').reset();
        }
    } catch (error) {
        showNotification(error.message || '启动回测失败', 'error');
    }
}

// 显示进度卡片
function showProgressCard(taskName, taskId) {
    const card = document.getElementById('backtest-progress-card');
    card.style.display = 'block';
    
    document.getElementById('progress-task-name').textContent = taskName;
    document.getElementById('progress-percentage').textContent = '0%';
    document.getElementById('progress-fill').style.width = '0%';
    document.getElementById('progress-status').textContent = '准备中...';
    document.getElementById('view-result-btn').style.display = 'none';
    
    card.scrollIntoView({ behavior: 'smooth' });
}

// 连接WebSocket
function connectWebSocket(taskId) {
    // 关闭现有连接
    if (currentWs) {
        currentWs.close();
    }
    
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/ws/backtest/${taskId}`;
    
    currentWs = new WebSocket(wsUrl);
    
    currentWs.onopen = () => {
        console.log('WebSocket已连接');
    };
    
    currentWs.onmessage = (event) => {
        try {
            const data = JSON.parse(event.data);
            handleWebSocketMessage(data);
        } catch (error) {
            console.error('解析WebSocket消息失败:', error);
        }
    };
    
    currentWs.onerror = (error) => {
        console.error('WebSocket错误:', error);
        showNotification('实时连接出错', 'error');
    };
    
    currentWs.onclose = () => {
        console.log('WebSocket已断开');
        currentWs = null;
    };
}

// 处理WebSocket消息
function handleWebSocketMessage(data) {
    console.log('收到消息:', data);
    
    switch (data.type) {
        case 'connected':
            document.getElementById('progress-status').textContent = '已连接，等待开始...';
            break;
            
        case 'status_update':
            updateProgress(data);
            break;
            
        case 'final':
            handleTaskComplete(data);
            break;
            
        case 'error':
            showNotification(data.message, 'error');
            break;
    }
}

// 更新进度
function updateProgress(data) {
    const progress = data.progress || 0;
    const status = data.status;
    
    document.getElementById('progress-percentage').textContent = `${progress}%`;
    document.getElementById('progress-fill').style.width = `${progress}%`;
    
    let statusText = '运行中...';
    if (status === 'pending') statusText = '等待中...';
    else if (status === 'running') statusText = `运行中... (${progress}%)`;
    else if (status === 'completed') statusText = '已完成 ✓';
    else if (status === 'failed') statusText = '失败 ✗';
    
    document.getElementById('progress-status').textContent = statusText;
    
    if (data.error) {
        document.getElementById('progress-status').textContent = `错误: ${data.error}`;
    }
}

// 处理任务完成
function handleTaskComplete(data) {
    showNotification('回测任务已完成', 'success');
    document.getElementById('view-result-btn').style.display = 'inline-block';
    
    if (currentWs) {
        currentWs.close();
    }
    
    // 刷新仪表盘数据
    if (appState.currentPage === 'dashboard') {
        loadDashboard();
    }
}

// 加载历史记录
async function loadHistory() {
    try {
        const response = await apiRequest('/backtest/history');
        if (response.success) {
            appState.tasks = response.data;
            displayHistory();
        }
    } catch (error) {
        showNotification('加载历史记录失败', 'error');
        console.error(error);
    }
}

// 刷新历史记录
document.getElementById('refresh-history-btn')?.addEventListener('click', loadHistory);

function displayHistory() {
    const container = document.getElementById('history-list');
    
    if (appState.tasks.length === 0) {
        container.innerHTML = '<p style="color: var(--text-secondary);">暂无历史记录</p>';
        return;
    }

    container.innerHTML = appState.tasks.map(task => `
        <div class="task-item" onclick="viewTaskResult('${task.id}')" style="cursor: pointer;">
            <div class="task-header">
                <span class="task-name">${task.name}</span>
                <span class="task-status status-${task.status}">${getStatusText(task.status)}</span>
            </div>
            <div class="task-meta">
                <span>📅 创建: ${formatDate(task.created_at)}</span>
                ${task.started_at ? `<span>▶️ 开始: ${formatDate(task.started_at)}</span>` : ''}
                ${task.completed_at ? `<span>✓ 完成: ${formatDate(task.completed_at)}</span>` : ''}
                <span>⏱️ 进度: ${task.progress}%</span>
            </div>
            ${task.error ? `<div style="color: var(--danger-color); font-size: 12px; margin-top: 8px;">❌ 错误: ${task.error}</div>` : ''}
            ${task.status === 'completed' ? `<div style="color: var(--primary-color); font-size: 12px; margin-top: 8px;">💡 点击查看详细结果和图表</div>` : ''}
        </div>
    `).join('');
}
