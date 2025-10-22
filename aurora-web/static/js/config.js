// Aurora Web - 配置管理

let isEditMode = false;
let currentConfigFilename = null;
let isFormMode = true; // true: 表单模式, false: TOML文本模式

// 初始化配置管理
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('new-config-btn')?.addEventListener('click', () => showConfigEditor());
    document.getElementById('save-config-btn')?.addEventListener('click', saveConfig);
    document.getElementById('validate-config-btn')?.addEventListener('click', validateConfig);
    document.getElementById('cancel-config-btn')?.addEventListener('click', hideConfigEditor);
    document.getElementById('import-config-btn')?.addEventListener('click', () => document.getElementById('config-import-file').click());
    document.getElementById('config-import-file')?.addEventListener('change', handleConfigImport);
    document.getElementById('toggle-editor-mode')?.addEventListener('click', toggleEditorMode);
});

// 加载配置列表
async function loadConfigs() {
    try {
        const response = await apiRequest('/config');
        if (response.success) {
            appState.configs = response.data;
            displayConfigs();
        }
    } catch (error) {
        showNotification('加载配置列表失败', 'error');
        console.error(error);
    }
}

function displayConfigs() {
    const container = document.getElementById('config-list');
    
    if (appState.configs.length === 0) {
        container.innerHTML = '<p style="color: var(--text-secondary);">暂无配置文件</p>';
        return;
    }

    container.innerHTML = appState.configs.map(config => `
        <div class="file-item">
            <div class="file-header">
                <span class="file-name">📄 ${config.filename}</span>
                <div>
                    <button class="btn btn-secondary" onclick="editConfig('${config.filename}')" style="padding: 6px 12px; font-size: 12px;">编辑</button>
                    <button class="btn btn-danger" onclick="deleteConfig('${config.filename}')" style="padding: 6px 12px; font-size: 12px;">删除</button>
                </div>
            </div>
            <div class="file-meta">
                <span>📅 修改时间: ${config.modified}</span>
            </div>
        </div>
    `).join('');
}

// 显示配置编辑器
function showConfigEditor(filename = null) {
    isEditMode = !!filename;
    currentConfigFilename = filename;
    isFormMode = true; // 默认使用表单模式
    
    const card = document.getElementById('config-editor-card');
    const filenameInput = document.getElementById('config-filename');
    const contentArea = document.getElementById('config-content');
    const formMode = document.getElementById('config-form-mode');
    const textMode = document.getElementById('config-text-mode');
    const toggleBtn = document.getElementById('toggle-editor-mode');
    
    card.style.display = 'block';
    formMode.style.display = 'block';
    textMode.style.display = 'none';
    toggleBtn.textContent = '📝 切换到文本模式';
    
    if (isEditMode) {
        filenameInput.value = filename;
        filenameInput.disabled = true;
        loadConfigContent(filename);
    } else {
        filenameInput.value = '';
        filenameInput.disabled = false;
        contentArea.value = getDefaultConfig();
        // 使用默认值初始化表单
        parseTomlToForm(getDefaultConfig());
    }
    
    card.scrollIntoView({ behavior: 'smooth' });
}

function hideConfigEditor() {
    document.getElementById('config-editor-card').style.display = 'none';
    document.getElementById('config-validation-result').classList.remove('show');
}

// 加载配置内容
async function loadConfigContent(filename) {
    try {
        const response = await apiRequest(`/config/${filename}`);
        if (response.success) {
            const content = response.data;
            document.getElementById('config-content').value = content;
            // 解析到表单
            parseTomlToForm(content);
        }
    } catch (error) {
        showNotification('加载配置内容失败', 'error');
        console.error(error);
    }
}

// 编辑配置
async function editConfig(filename) {
    showConfigEditor(filename);
}

// 保存配置
async function saveConfig() {
    const filename = document.getElementById('config-filename').value.trim();
    
    // 根据当前模式获取配置内容
    let content;
    if (isFormMode) {
        content = formToToml();
    } else {
        content = document.getElementById('config-content').value;
    }
    
    if (!filename) {
        showNotification('请输入文件名', 'error');
        return;
    }
    
    if (!filename.endsWith('.toml')) {
        showNotification('文件名必须以.toml结尾', 'error');
        return;
    }
    
    try {
        if (isEditMode) {
            // 更新现有配置
            await apiRequest(`/config/${filename}`, {
                method: 'PUT',
                body: JSON.stringify({ content })
            });
            showNotification('配置已更新', 'success');
        } else {
            // 创建新配置
            await apiRequest('/config', {
                method: 'POST',
                body: JSON.stringify({ filename, content })
            });
            showNotification('配置已创建', 'success');
        }
        
        hideConfigEditor();
        loadConfigs();
    } catch (error) {
        showNotification(error.message || '保存配置失败', 'error');
    }
}

// 验证配置
async function validateConfig() {
    const content = document.getElementById('config-content').value;
    const resultDiv = document.getElementById('config-validation-result');
    
    try {
        const response = await apiRequest('/config/validate', {
            method: 'POST',
            body: JSON.stringify({ content })
        });
        
        if (response.success) {
            const validation = response.data;
            
            if (validation.valid) {
                resultDiv.className = 'validation-result success show';
                resultDiv.textContent = '✅ 配置验证成功！';
            } else {
                resultDiv.className = 'validation-result error show';
                resultDiv.innerHTML = `
                    <strong>❌ 配置验证失败：</strong>
                    <ul>${validation.errors.map(err => `<li>${err}</li>`).join('')}</ul>
                `;
            }
        }
    } catch (error) {
        resultDiv.className = 'validation-result error show';
        resultDiv.textContent = `❌ 验证失败: ${error.message}`;
    }
}

// 删除配置
async function deleteConfig(filename) {
    if (!confirm(`确定要删除配置文件 "${filename}" 吗？`)) {
        return;
    }
    
    try {
        await apiRequest(`/config/${filename}`, {
            method: 'DELETE'
        });
        showNotification('配置已删除', 'success');
        loadConfigs();
    } catch (error) {
        showNotification('删除配置失败', 'error');
    }
}

// 获取默认配置模板
function getDefaultConfig() {
    return `# Aurora 配置文件

[data_source]
provider = "binance"
timeout = 30
max_retries = 3

[[strategies]]
name = "MA交叉策略"
strategy_type = "ma-crossover"
enabled = true

[strategies.parameters]
short = 10
long = 30

[portfolio]
initial_cash = 10000.0
commission = 0.001
slippage = 0.0005

[portfolio.risk_rules]
stop_loss_pct = 2.0
take_profit_pct = 5.0
max_drawdown_pct = 15.0

[backtest]
start_time = "2024-01-01T00:00:00Z"
end_time = "2024-12-31T23:59:59Z"

[backtest.pricing_mode]
mode = "close"
`;
}

// 切换编辑模式（表单 <-> TOML文本）
function toggleEditorMode() {
    isFormMode = !isFormMode;
    const formMode = document.getElementById('config-form-mode');
    const textMode = document.getElementById('config-text-mode');
    const toggleBtn = document.getElementById('toggle-editor-mode');
    
    if (isFormMode) {
        // 从 TOML 文本切换到表单
        const tomlText = document.getElementById('config-content').value;
        if (tomlText.trim()) {
            parseTomlToForm(tomlText);
        }
        formMode.style.display = 'block';
        textMode.style.display = 'none';
        toggleBtn.textContent = '📝 切换到文本模式';
    } else {
        // 从表单切换到 TOML 文本
        const tomlText = formToToml();
        document.getElementById('config-content').value = tomlText;
        formMode.style.display = 'none';
        textMode.style.display = 'block';
        toggleBtn.textContent = '📋 切换到表单模式';
    }
}

// 将表单数据转换为 TOML
function formToToml() {
    const provider = document.getElementById('form-provider').value;
    const timeout = document.getElementById('form-timeout').value;
    const retries = document.getElementById('form-retries').value;
    
    const strategyName = document.getElementById('form-strategy-name').value;
    const strategyType = document.getElementById('form-strategy-type').value;
    const strategyEnabled = document.getElementById('form-strategy-enabled').checked;
    const maShort = document.getElementById('form-ma-short').value;
    const maLong = document.getElementById('form-ma-long').value;
    
    const initialCash = document.getElementById('form-initial-cash').value;
    const commission = (parseFloat(document.getElementById('form-commission').value) / 100).toFixed(4);
    const slippage = (parseFloat(document.getElementById('form-slippage').value) / 100).toFixed(4);
    
    const stopLoss = document.getElementById('form-stop-loss').value;
    const takeProfit = document.getElementById('form-take-profit').value;
    const maxDrawdown = document.getElementById('form-max-drawdown').value;
    
    const startTime = document.getElementById('form-start-time').value;
    const endTime = document.getElementById('form-end-time').value;
    const pricingMode = document.getElementById('form-pricing-mode').value;
    
    return `# Aurora 配置文件

[data_source]
provider = "${provider}"
timeout = ${timeout}
max_retries = ${retries}

[[strategies]]
name = "${strategyName}"
strategy_type = "${strategyType}"
enabled = ${strategyEnabled}

[strategies.parameters]
short = ${maShort}
long = ${maLong}

[portfolio]
initial_cash = ${initialCash}
commission = ${commission}
slippage = ${slippage}

[portfolio.risk_rules]
stop_loss_pct = ${stopLoss}
take_profit_pct = ${takeProfit}
max_drawdown_pct = ${maxDrawdown}

[backtest]
start_time = "${startTime}:00Z"
end_time = "${endTime}:59Z"

[backtest.pricing_mode]
mode = "${pricingMode}"
`;
}

// 解析 TOML 到表单（简单版本）
function parseTomlToForm(tomlText) {
    try {
        // 简单的 TOML 解析（实际项目中应该使用专业的 TOML 解析库）
        const lines = tomlText.split('\n');
        
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i].trim();
            if (!line || line.startsWith('#')) continue;
            
            if (line.includes('provider =')) {
                const value = line.match(/provider\s*=\s*"([^"]*)"/)?.[1];
                if (value) document.getElementById('form-provider').value = value;
            } else if (line.includes('timeout =')) {
                const value = line.match(/timeout\s*=\s*(\d+)/)?.[1];
                if (value) document.getElementById('form-timeout').value = value;
            } else if (line.includes('max_retries =')) {
                const value = line.match(/max_retries\s*=\s*(\d+)/)?.[1];
                if (value) document.getElementById('form-retries').value = value;
            } else if (line.includes('name =') && !line.includes('strategy_type')) {
                const value = line.match(/name\s*=\s*"([^"]*)"/)?.[1];
                if (value) document.getElementById('form-strategy-name').value = value;
            } else if (line.includes('strategy_type =')) {
                const value = line.match(/strategy_type\s*=\s*"([^"]*)"/)?.[1];
                if (value) document.getElementById('form-strategy-type').value = value;
            } else if (line.includes('enabled =')) {
                const value = line.match(/enabled\s*=\s*(true|false)/)?.[1];
                if (value) document.getElementById('form-strategy-enabled').checked = value === 'true';
            } else if (line.includes('short =')) {
                const value = line.match(/short\s*=\s*(\d+)/)?.[1];
                if (value) document.getElementById('form-ma-short').value = value;
            } else if (line.includes('long =')) {
                const value = line.match(/long\s*=\s*(\d+)/)?.[1];
                if (value) document.getElementById('form-ma-long').value = value;
            } else if (line.includes('initial_cash =')) {
                const value = line.match(/initial_cash\s*=\s*([\d.]+)/)?.[1];
                if (value) document.getElementById('form-initial-cash').value = value;
            } else if (line.includes('commission =')) {
                const value = line.match(/commission\s*=\s*([\d.]+)/)?.[1];
                if (value) document.getElementById('form-commission').value = (parseFloat(value) * 100).toFixed(2);
            } else if (line.includes('slippage =')) {
                const value = line.match(/slippage\s*=\s*([\d.]+)/)?.[1];
                if (value) document.getElementById('form-slippage').value = (parseFloat(value) * 100).toFixed(2);
            } else if (line.includes('stop_loss_pct =')) {
                const value = line.match(/stop_loss_pct\s*=\s*([\d.]+)/)?.[1];
                if (value) document.getElementById('form-stop-loss').value = value;
            } else if (line.includes('take_profit_pct =')) {
                const value = line.match(/take_profit_pct\s*=\s*([\d.]+)/)?.[1];
                if (value) document.getElementById('form-take-profit').value = value;
            } else if (line.includes('max_drawdown_pct =')) {
                const value = line.match(/max_drawdown_pct\s*=\s*([\d.]+)/)?.[1];
                if (value) document.getElementById('form-max-drawdown').value = value;
            } else if (line.includes('start_time =')) {
                const value = line.match(/start_time\s*=\s*"([^"]*)"/)?.[1];
                if (value) {
                    const dateStr = value.replace('Z', '').replace(':00Z', '').substring(0, 16);
                    document.getElementById('form-start-time').value = dateStr;
                }
            } else if (line.includes('end_time =')) {
                const value = line.match(/end_time\s*=\s*"([^"]*)"/)?.[1];
                if (value) {
                    const dateStr = value.replace('Z', '').replace(':59Z', '').substring(0, 16);
                    document.getElementById('form-end-time').value = dateStr;
                }
            } else if (line.includes('mode =') && !line.includes('pricing_mode')) {
                const value = line.match(/mode\s*=\s*"([^"]*)"/)?.[1];
                if (value) document.getElementById('form-pricing-mode').value = value;
            }
        }
    } catch (error) {
        console.error('解析 TOML 失败:', error);
        showNotification('解析 TOML 失败，请切换到文本模式手动编辑', 'error');
    }
}

// 处理配置文件导入
async function handleConfigImport(event) {
    const file = event.target.files[0];
    if (!file) return;
    
    if (!file.name.endsWith('.toml')) {
        showNotification('请选择 .toml 文件', 'error');
        return;
    }
    
    try {
        const content = await file.text();
        
        if (isFormMode) {
            // 表单模式：解析并填充表单
            parseTomlToForm(content);
            showNotification('配置文件已导入到表单', 'success');
        } else {
            // 文本模式：直接填充到文本框
            document.getElementById('config-content').value = content;
            showNotification('配置文件已导入', 'success');
        }
        
        // 建议使用导入的文件名
        if (!isEditMode) {
            const suggestedName = file.name;
            document.getElementById('config-filename').value = suggestedName;
        }
    } catch (error) {
        showNotification('读取文件失败: ' + error.message, 'error');
    }
    
    // 清空文件选择器，允许重复导入同一文件
    event.target.value = '';
}
