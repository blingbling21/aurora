/**
 * Copyright 2025 blingbling21
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

'use client';

import { useState, useRef } from 'react';
import {
  PageHeader,
  Button,
  Card,
  Input,
  Textarea,
} from '@/components/ui';
import {
  AuroraConfig,
  createDefaultAuroraConfig,
  DataSourceConfig,
  StrategyConfig,
  PortfolioConfig,
  LoggingConfig,
  BacktestSettings,
  LiveConfig,
} from '@/types/config-schema';
import { readTOMLFile, stringifyTOML, validateTOML } from '@/lib/utils/toml';
import { useNotificationStore } from '@/lib/store';
import { configApi } from '@/lib/api';
import { getCurrentTimezone } from '@/constants';
import {
  DataSourceSection,
  StrategiesSection,
  PortfolioSection,
  LoggingSection,
  BacktestSection,
  LiveSection,
} from './ConfigSections';
import { ConfigList } from '@/components/dashboard/ConfigList';

/**
 * 配置管理页面
 * 
 * 管理和编辑回测配置文件
 */
export default function ConfigPage() {
  // 状态管理
  const [isEditing, setIsEditing] = useState(false);
  const [editMode, setEditMode] = useState<'form' | 'text'>('form');
  const [config, setConfig] = useState<AuroraConfig>(createDefaultAuroraConfig());
  const [tomlText, setTomlText] = useState('');
  const [filename, setFilename] = useState('config.toml');
  const [isValidating, setIsValidating] = useState(false);
  const [refreshTrigger, setRefreshTrigger] = useState(0);
  
  // 文件输入引用
  const fileInputRef = useRef<HTMLInputElement>(null);
  
  // 通知store
  const { addNotification } = useNotificationStore();

  /**
   * 从服务器加载配置文件
   */
  const handleLoadConfig = async (selectedFilename: string) => {
    try {
      // 获取配置文件内容
      const response = await configApi.get(selectedFilename);
      
      if (response.success && response.data) {
        // 解析TOML内容
        const result = await validateTOML(response.data);
        
        if (result.valid && result.config) {
          setConfig(result.config);
          setTomlText(response.data);
          setFilename(selectedFilename);
          setIsEditing(true);
          setEditMode('form');
          
          addNotification({
            type: 'success',
            message: `成功加载配置文件: ${selectedFilename}`,
          });
        } else {
          throw new Error('配置文件格式错误');
        }
      } else {
        throw new Error(response.error || '加载配置文件失败');
      }
    } catch (error) {
      addNotification({
        type: 'error',
        message: error instanceof Error ? error.message : '加载配置文件失败',
      });
    }
  };

  /**
   * 处理TOML文件导入
   */
  const handleImportTOML = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    try {
      // 读取并解析TOML文件
      const parsedConfig = await readTOMLFile(file);
      
      // 更新配置状态
      setConfig(parsedConfig);
      
      // 如果在文本模式,也更新文本内容
      if (editMode === 'text') {
        const text = await stringifyTOML(parsedConfig);
        setTomlText(text);
      }
      
      // 设置文件名
      setFilename(file.name);
      
      // 开启编辑模式
      setIsEditing(true);
      
      // 显示成功通知
      addNotification({
        type: 'success',
        message: `成功导入配置文件: ${file.name}`,
      });
    } catch (error) {
      // 显示错误通知
      addNotification({
        type: 'error',
        message: error instanceof Error ? error.message : '导入配置文件失败',
      });
    }
    
    // 重置文件输入
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  /**
   * 处理配置保存
   */
  const handleSave = async () => {
    // 验证文件名
    if (!filename.trim()) {
      addNotification({
        type: 'error',
        message: '请输入文件名',
      });
      return;
    }

    if (!filename.endsWith('.toml')) {
      addNotification({
        type: 'error',
        message: '文件名必须以.toml结尾',
      });
      return;
    }

    try {
      // 获取要保存的内容
      let contentToSave = tomlText;
      
      // 如果在表单模式,先转换为TOML文本
      if (editMode === 'form') {
        // 确保回测配置中有时区字段的默认值
        const configToSave = { ...config };
        if (configToSave.backtest && !configToSave.backtest.timezone) {
          configToSave.backtest = {
            ...configToSave.backtest,
            timezone: getCurrentTimezone(),
          };
        }
        contentToSave = await stringifyTOML(configToSave);
      }

      // 先检查配置是否已存在
      const listResponse = await configApi.list();
      const exists = listResponse.success && listResponse.data?.some(
        item => item.filename === filename
      );

      // 根据是否存在选择创建或更新
      const response = exists
        ? await configApi.update(filename, { content: contentToSave })
        : await configApi.create({ filename, content: contentToSave });

      if (response.success) {
        addNotification({
          type: 'success',
          message: `配置${exists ? '更新' : '保存'}成功`,
        });
        
        // 刷新配置列表
        setRefreshTrigger(prev => prev + 1);
        
        // 关闭编辑器
        setIsEditing(false);
      } else {
        addNotification({
          type: 'error',
          message: response.error || '保存配置失败',
        });
      }
    } catch (error) {
      addNotification({
        type: 'error',
        message: error instanceof Error ? error.message : '保存配置失败',
      });
    }
  };

  /**
   * 处理配置验证
   */
  const handleValidate = async () => {
    setIsValidating(true);
    
    try {
      let textToValidate = tomlText;
      
      // 如果在表单模式,先转换为TOML文本
      if (editMode === 'form') {
        textToValidate = await stringifyTOML(config);
      }
      
      // 调用后端API验证TOML
      const { configApi } = await import('@/lib/api');
      const response = await configApi.validate(textToValidate);
      
      if (response.success && response.data) {
        if (response.data.valid) {
          addNotification({
            type: 'success',
            message: '配置验证通过!',
          });
        } else {
          addNotification({
            type: 'error',
            message: `配置验证失败: ${response.data.errors?.join(', ') || '未知错误'}`,
          });
        }
      } else {
        addNotification({
          type: 'error',
          message: response.error || '配置验证失败',
        });
      }
    } catch (error) {
      addNotification({
        type: 'error',
        message: error instanceof Error ? error.message : '配置验证失败',
      });
    } finally {
      setIsValidating(false);
    }
  };

  /**
   * 处理模式切换
   */
  const handleModeSwitch = async () => {
    try {
      if (editMode === 'form') {
        // 从表单模式切换到文本模式
        const text = await stringifyTOML(config);
        setTomlText(text);
        setEditMode('text');
      } else {
        // 从文本模式切换到表单模式
        const result = await validateTOML(tomlText);
        if (result.valid && result.config) {
          setConfig(result.config);
          setEditMode('form');
        } else {
          addNotification({
            type: 'error',
            message: '请先修正TOML文本中的错误',
          });
        }
      }
    } catch (error) {
      addNotification({
        type: 'error',
        message: error instanceof Error ? error.message : '模式切换失败',
      });
    }
  };

  /**
   * 更新配置对象的辅助函数
   */
  const updateConfig = <K extends keyof AuroraConfig>(
    key: K,
    value: AuroraConfig[K]
  ) => {
    setConfig((prev) => ({ ...prev, [key]: value }));
  };

  return (
    <div>
      {/* 页面头部 */}
      <PageHeader
        icon="⚙️"
        title="配置管理"
        action={
          <div className="flex gap-3">
            <Button onClick={() => {
              setConfig(createDefaultAuroraConfig());
              setIsEditing(true);
            }}>
              + 新建配置
            </Button>
            <Button variant="secondary">🔄 刷新</Button>
          </div>
        }
      />

      <div className="grid grid-cols-1 gap-6">
        {/* 配置编辑器 */}
        <Card title="配置编辑器" className="mt-6">
          {!isEditing ? (
            <div className="text-center py-12">
              <p className="text-gray-500 mb-4">选择或创建一个配置文件以开始编辑</p>
              <div className="flex gap-3 justify-center">
                <Button onClick={() => {
                  setConfig(createDefaultAuroraConfig());
                  setIsEditing(true);
                }}>
                  + 新建配置
                </Button>
                <Button 
                  variant="secondary"
                  onClick={() => fileInputRef.current?.click()}
                >
                  📁 导入配置
                </Button>
                <input
                  ref={fileInputRef}
                  type="file"
                  accept=".toml"
                  className="hidden"
                  onChange={handleImportTOML}
                />
              </div>
            </div>
          ) : (
            <>
              <div className="mb-4 flex gap-3">
                <input
                  ref={fileInputRef}
                  type="file"
                  accept=".toml"
                  className="hidden"
                  onChange={handleImportTOML}
                />
                <Button
                  variant="secondary"
                  onClick={() => fileInputRef.current?.click()}
                >
                  📁 导入 TOML
                </Button>
                <Button
                  variant="secondary"
                  onClick={handleModeSwitch}
                >
                  {editMode === 'form' ? '📝 文本模式' : '📋 表单模式'}
                </Button>
              </div>

              <div className="mb-4">
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  文件名:
                </label>
                <Input
                  type="text"
                  value={filename}
                  onChange={(e) => setFilename(e.target.value)}
                  placeholder="example.toml"
                  className="w-full"
                />
              </div>

              {editMode === 'form' ? (
                <ConfigForm config={config} updateConfig={updateConfig} />
              ) : (
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    配置内容 (TOML):
                  </label>
                  <Textarea
                    value={tomlText}
                    onChange={(e) => setTomlText(e.target.value)}
                    rows={20}
                    placeholder="在此输入TOML配置..."
                    className="w-full font-mono text-sm"
                  />
                </div>
              )}

              <div className="mt-6 flex gap-3">
                <Button onClick={handleSave}>💾 保存</Button>
                <Button 
                  variant="secondary"
                  onClick={handleValidate}
                  disabled={isValidating}
                >
                  {isValidating ? '验证中...' : '✓ 验证'}
                </Button>
                <Button variant="secondary" onClick={() => setIsEditing(false)}>
                  ✕ 取消
                </Button>
              </div>
            </>
          )}
        </Card>

        {/* 配置文件列表 */}
        <ConfigList 
          onSelect={handleLoadConfig}
          refreshTrigger={refreshTrigger}
        />
      </div>
    </div>
  );
}

/**
 * 配置表单组件
 */
interface ConfigFormProps {
  config: AuroraConfig;
  updateConfig: <K extends keyof AuroraConfig>(key: K, value: AuroraConfig[K]) => void;
}

function ConfigForm({ config, updateConfig }: ConfigFormProps) {
  return (
    <div className="space-y-6">
      {/* 数据源配置 */}
      <DataSourceSection 
        config={config.data_source}
        onChange={(value: DataSourceConfig) => updateConfig('data_source', value)}
      />

      {/* 策略配置 */}
      <StrategiesSection
        strategies={config.strategies}
        onChange={(value: StrategyConfig[]) => updateConfig('strategies', value)}
      />

      {/* 投资组合配置 */}
      <PortfolioSection
        config={config.portfolio}
        onChange={(value: PortfolioConfig) => updateConfig('portfolio', value)}
      />

      {/* 日志配置 */}
      <LoggingSection
        config={config.logging}
        onChange={(value: LoggingConfig) => updateConfig('logging', value)}
      />

      {/* 回测配置 */}
      <BacktestSection
        config={config.backtest}
        onChange={(value: BacktestSettings | undefined) => updateConfig('backtest', value)}
      />

      {/* 实时交易配置 */}
      <LiveSection
        config={config.live}
        onChange={(value: LiveConfig | undefined) => updateConfig('live', value)}
      />
    </div>
  );
}
