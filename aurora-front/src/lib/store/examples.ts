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

/**
 * Zod数据验证和Zustand状态管理使用示例
 * 
 * 本文件展示了如何在Aurora项目中使用Zod进行数据验证和Zustand进行状态管理
 */

import { 
  BacktestTaskSchema, 
  validateBacktestTask
} from '@/types';
import { 
  useBacktestTaskStore,
  useBacktestResultStore,
  useNotificationStore 
} from '@/lib/store';

// ============================================================================
// 示例1: 使用Zod进行数据验证
// ============================================================================

/**
 * 示例：验证从API获取的回测任务数据
 */
export function exampleValidateApiData() {
  // 模拟从API获取的数据
  const apiData = {
    id: '123',
    name: 'My Backtest',
    status: 'running',
    config: 'config.toml',
    dataFile: 'data.csv',
    progress: 75,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };

  // 方法1: 使用schema.safeParse进行安全解析
  const result = BacktestTaskSchema.safeParse(apiData);
  
  if (result.success) {
    // 数据验证成功，可以安全使用
    console.log('Valid task:', result.data);
    return result.data;
  } else {
    // 数据验证失败，处理错误
    console.error('Validation errors:', result.error.issues);
    return null;
  }

  // 方法2: 使用验证函数
  const validationResult = validateBacktestTask(apiData);
  
  if (validationResult.success) {
    console.log('Valid task:', validationResult.data);
    return validationResult.data;
  } else {
    const errorMessage = validationResult.errors?.map(e => e.message).join(', ') || 'Unknown validation error';
    console.error('Validation failed:', errorMessage);
    return null;
  }
}

// ============================================================================
// 示例2: 在React组件中使用Zustand状态管理
// ============================================================================

/**
 * 示例：在组件中使用回测任务store
 */
export function ExampleBacktestTaskComponent() {
  // 从store中获取状态和actions
  const { 
    tasks, 
    selectedTaskId, 
    isLoading,
    addTask,
    updateTask,
    selectTask,
    setLoading 
  } = useBacktestTaskStore();

  // 添加新任务
  const handleAddTask = async () => {
    setLoading(true);
    
    try {
      // 创建新任务数据
      const newTask = {
        id: Date.now().toString(),
        name: 'New Backtest',
        status: 'pending' as const,
        config: 'config.toml',
        dataFile: 'data.csv',
        progress: 0,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };

      // 验证数据
      const validation = validateBacktestTask(newTask);
      
      if (validation.success) {
        // 添加到store
        addTask(validation.data);
      } else {
        console.error('Invalid task data');
      }
    } finally {
      setLoading(false);
    }
  };

  // 更新任务状态
  const handleUpdateTaskStatus = (taskId: string) => {
    updateTask(taskId, {
      status: 'running',
      progress: 50,
      updatedAt: new Date().toISOString(),
    });
  };

  return {
    tasks,
    selectedTaskId,
    isLoading,
    handleAddTask,
    handleUpdateTaskStatus,
    selectTask,
  };
}

// ============================================================================
// 示例3: 使用通知store显示消息
// ============================================================================

/**
 * 示例：在组件中使用通知系统
 */
export function ExampleNotificationUsage() {
  const { showSuccess, showError, showWarning, showInfo } = useNotificationStore();

  // 成功通知
  const handleSuccess = () => {
    showSuccess('回测任务创建成功！', 3000);
  };

  // 错误通知
  const handleError = () => {
    showError('回测任务执行失败，请检查配置', 5000);
  };

  // 警告通知
  const handleWarning = () => {
    showWarning('数据文件较大，下载可能需要一些时间', 4000);
  };

  // 信息通知
  const handleInfo = () => {
    showInfo('正在加载配置文件...', 2000);
  };

  return {
    handleSuccess,
    handleError,
    handleWarning,
    handleInfo,
  };
}

// ============================================================================
// 示例4: 结合Zod和Zustand进行表单验证
// ============================================================================

/**
 * 示例：表单提交时的数据验证和状态更新
 */
export function ExampleFormSubmission() {
  const { addTask, setError } = useBacktestTaskStore();
  const { showSuccess, showError } = useNotificationStore();

  const handleSubmit = async (formData: unknown) => {
    // 验证表单数据
    const validation = validateBacktestTask(formData);

    if (!validation.success) {
      // 验证失败，显示错误信息
      const errorMessage = validation.errors?.map(e => e.message).join(', ') || '验证失败';
      setError(errorMessage);
      showError(`数据验证失败: ${errorMessage}`);
      return;
    }

    try {
      // 验证成功，提交数据
      // const response = await api.createTask(validation.data);
      
      // 更新store
      addTask(validation.data);
      
      // 显示成功通知
      showSuccess('回测任务创建成功！');
    } catch {
      // 处理API错误
      showError('创建任务失败，请重试');
    }
  };

  return { handleSubmit };
}

// ============================================================================
// 示例5: 使用多个store协同工作
// ============================================================================

/**
 * 示例：在一个功能中使用多个store
 */
export function ExampleMultipleStores() {
  const { updateTask } = useBacktestTaskStore();
  const { setResult } = useBacktestResultStore();
  const { showSuccess, showError, showInfo } = useNotificationStore();

  const runBacktest = async (taskId: string) => {
    try {
      // 更新任务状态为运行中
      updateTask(taskId, {
        status: 'running',
        progress: 0,
        updatedAt: new Date().toISOString(),
      });

      showInfo('回测任务开始执行...');

      // 模拟回测执行
      // const result = await api.runBacktest(taskId);

      // 更新任务状态为完成
      updateTask(taskId, {
        status: 'completed',
        progress: 100,
        updatedAt: new Date().toISOString(),
      });

      // 保存回测结果
      // setResult(taskId, result);

      showSuccess('回测任务执行完成！');
    } catch {
      // 更新任务状态为失败
      updateTask(taskId, {
        status: 'failed',
        updatedAt: new Date().toISOString(),
      });

      showError('回测任务执行失败');
    }
  };

  return { runBacktest, setResult };
}

// ============================================================================
// 示例6: Store的选择器模式
// ============================================================================

/**
 * 示例：使用选择器优化性能
 */
export function ExampleStoreSelectors() {
  // 只订阅需要的状态，避免不必要的重渲染
  const tasks = useBacktestTaskStore((state) => state.tasks);
  const isLoading = useBacktestTaskStore((state) => state.isLoading);
  const addTask = useBacktestTaskStore((state) => state.addTask);

  // 使用计算属性
  const runningTasks = tasks.filter((task) => task.status === 'running');
  const completedTasks = tasks.filter((task) => task.status === 'completed');

  return {
    runningTasks,
    completedTasks,
    isLoading,
    addTask,
  };
}
