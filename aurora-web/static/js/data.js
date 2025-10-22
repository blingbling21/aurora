// Aurora Web - 数据管理

// 初始化数据管理
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('fetch-data-form')?.addEventListener('submit', handleFetchData);
    document.getElementById('preview-filename-btn')?.addEventListener('click', previewFilename);
    document.getElementById('refresh-data-btn')?.addEventListener('click', loadDataFiles);
    
    // 交易对下拉框选择处理
    document.getElementById('data-symbol-select')?.addEventListener('change', (e) => {
        const value = e.target.value;
        if (value) {
            document.getElementById('data-symbol').value = value;
            updateFilenamePreview();
        }
    });
    
    // 监听表单输入变化，自动更新文件名预览
    ['data-exchange', 'data-symbol', 'data-interval', 'data-start-date', 'data-end-date'].forEach(id => {
        document.getElementById(id)?.addEventListener('change', updateFilenamePreview);
    });
});

// 加载数据文件列表
async function loadDataFiles() {
    try {
        const response = await apiRequest('/data/list');
        if (response.success) {
            appState.dataFiles = response.data;
            displayDataFiles();
        }
    } catch (error) {
        showNotification('加载数据文件列表失败', 'error');
        console.error(error);
    }
}

function displayDataFiles() {
    const container = document.getElementById('data-list');
    
    if (appState.dataFiles.length === 0) {
        container.innerHTML = '<p style="color: var(--text-secondary);">暂无数据文件</p>';
        return;
    }

    container.innerHTML = appState.dataFiles.map(file => `
        <div class="file-item">
            <div class="file-header">
                <span class="file-name">📊 ${file.filename}</span>
                <div>
                    <button class="btn btn-danger" onclick="deleteDataFile('${file.filename}')" style="padding: 6px 12px; font-size: 12px;">删除</button>
                </div>
            </div>
            <div class="file-meta">
                <span>📏 大小: ${formatFileSize(file.size)}</span>
                <span>📅 修改时间: ${file.modified}</span>
                ${file.record_count ? `<span>📈 记录数: ${file.record_count}</span>` : ''}
            </div>
        </div>
    `).join('');
}

// 删除数据文件
async function deleteDataFile(filename) {
    if (!confirm(`确定要删除数据文件 "${filename}" 吗？`)) {
        return;
    }
    
    try {
        await apiRequest(`/data/${filename}`, {
            method: 'DELETE'
        });
        showNotification('数据文件已删除', 'success');
        loadDataFiles();
    } catch (error) {
        showNotification('删除数据文件失败', 'error');
        console.error(error);
    }
}

// 生成文件名
function generateFilename() {
    const exchange = document.getElementById('data-exchange').value;
    const symbol = document.getElementById('data-symbol').value;
    const interval = document.getElementById('data-interval').value;
    const startDate = document.getElementById('data-start-date').value;
    const endDate = document.getElementById('data-end-date').value;
    
    if (!exchange || !symbol || !interval || !startDate || !endDate) {
        return '';
    }
    
    const formattedStart = startDate.replace(/-/g, '');
    const formattedEnd = endDate.replace(/-/g, '');
    
    return `${exchange.toLowerCase()}_${symbol.toLowerCase()}_${interval}_${formattedStart}_to_${formattedEnd}.csv`;
}

// 更新文件名预览
function updateFilenamePreview() {
    const filename = generateFilename();
    const filenameInput = document.getElementById('data-filename');
    if (filename) {
        filenameInput.value = filename;
    } else {
        filenameInput.value = '';
    }
}

// 预览文件名
function previewFilename() {
    updateFilenamePreview();
    const filename = document.getElementById('data-filename').value;
    if (filename) {
        showNotification(`文件将保存为: ${filename}`, 'info');
    } else {
        showNotification('请先填写所有必填字段', 'error');
    }
}

// 处理数据获取
async function handleFetchData(event) {
    event.preventDefault();
    
    const exchange = document.getElementById('data-exchange').value;
    const symbol = document.getElementById('data-symbol').value.trim().toUpperCase();
    const interval = document.getElementById('data-interval').value;
    const startDate = document.getElementById('data-start-date').value;
    const endDate = document.getElementById('data-end-date').value;
    
    // 验证交易对格式
    if (!validateSymbol(symbol)) {
        showNotification('交易对格式不正确。正确格式示例: BTCUSDT, ETHUSDT', 'error');
        return;
    }
    
    // 验证日期范围
    if (new Date(startDate) > new Date(endDate)) {
        showNotification('开始日期不能晚于结束日期', 'error');
        return;
    }
    
    // 验证日期不在未来
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    if (new Date(endDate) > today) {
        showNotification('结束日期不能晚于今天', 'error');
        return;
    }
    
    // 显示进度条
    const progressContainer = document.getElementById('fetch-progress');
    const statusText = document.getElementById('fetch-status');
    const percentageText = document.getElementById('fetch-percentage');
    const progressBar = document.getElementById('fetch-progress-bar');
    
    progressContainer.style.display = 'block';
    statusText.textContent = '正在准备下载...';
    percentageText.textContent = '0%';
    progressBar.style.width = '0%';
    
    try {
        // 模拟进度更新
        updateProgress(10, '连接到交易所...');
        
        const response = await apiRequest('/data/fetch', {
            method: 'POST',
            body: JSON.stringify({
                exchange,
                symbol,
                interval,
                start_date: startDate,
                end_date: endDate,
                filename: null // 让后端自动生成
            })
        });
        
        updateProgress(100, '下载完成！');
        
        if (response.success) {
            showNotification(response.data, 'success');
            
            // 重置表单
            document.getElementById('fetch-data-form').reset();
            document.getElementById('data-filename').value = '';
            document.getElementById('data-symbol-select').value = '';
            
            // 刷新文件列表
            setTimeout(() => {
                loadDataFiles();
                progressContainer.style.display = 'none';
            }, 2000);
        }
    } catch (error) {
        updateProgress(0, '下载失败');
        
        // 解析错误消息，提供更友好的提示
        let errorMessage = error.message || '获取数据失败';
        
        // 检查是否是交易对错误
        if (errorMessage.includes('Invalid symbol')) {
            errorMessage = `交易对 "${symbol}" 无效。请检查拼写是否正确（例如：BTCUSDT）`;
        } else if (errorMessage.includes('暂未支持')) {
            errorMessage = errorMessage; // 保持原错误消息
        }
        
        showNotification(errorMessage, 'error');
        setTimeout(() => {
            progressContainer.style.display = 'none';
        }, 3000);
    }
}

// 验证交易对格式
function validateSymbol(symbol) {
    // 基本格式验证：至少6个字符，只包含字母和数字
    if (!symbol || symbol.length < 6) {
        return false;
    }
    
    // 检查是否只包含大写字母和数字
    if (!/^[A-Z0-9]+$/.test(symbol)) {
        return false;
    }
    
    // 检查是否以常见的稳定币结尾
    const validEndings = ['USDT', 'BUSD', 'USDC', 'BTC', 'ETH', 'BNB'];
    const hasValidEnding = validEndings.some(ending => symbol.endsWith(ending));
    
    if (!hasValidEnding) {
        console.warn('交易对可能格式不正确，常见格式应以 USDT、BUSD、BTC 等结尾');
    }
    
    return true;
}

// 更新进度
function updateProgress(percentage, status) {
    const statusText = document.getElementById('fetch-status');
    const percentageText = document.getElementById('fetch-percentage');
    const progressBar = document.getElementById('fetch-progress-bar');
    
    if (statusText) statusText.textContent = status;
    if (percentageText) percentageText.textContent = `${percentage}%`;
    if (progressBar) progressBar.style.width = `${percentage}%`;
}
