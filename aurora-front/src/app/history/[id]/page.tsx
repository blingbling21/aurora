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

import { useState, useEffect } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { PageHeader, Button, Card } from '@/components/ui';
import { BacktestResult, BacktestTask } from '@/types';
import { backtestApi } from '@/lib/api';
import { useNotificationStore } from '@/lib/store/notificationStore';

/**
 * 回测详情页面
 * 
 * 显示单个回测任务的详细结果
 */
export default function BacktestDetailPage() {
  // 获取路由参数
  const params = useParams();
  const router = useRouter();
  const taskId = params.id as string;

  // 状态管理
  const [task, setTask] = useState<BacktestTask | null>(null);
  const [result, setResult] = useState<BacktestResult | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const { addNotification } = useNotificationStore();

  /**
   * 加载任务信息
   */
  const loadTask = async () => {
    try {
      const response = await backtestApi.list();
      if (response.success && response.data) {
        // 查找对应的任务
        const taskData = response.data.find((t) => t.id === taskId);
        if (taskData) {
          const convertedTask: BacktestTask = {
            id: taskData.id,
            name: taskData.name,
            status: taskData.status,
            config: taskData.config_path || '',
            dataFile: taskData.data_path || '',
            progress: taskData.progress,
            createdAt: taskData.created_at,
            updatedAt: taskData.completed_at || taskData.started_at || taskData.created_at,
          };
          setTask(convertedTask);
        } else {
          throw new Error('任务不存在');
        }
      } else {
        throw new Error(response.error || '加载任务失败');
      }
    } catch (error) {
      addNotification({
        type: 'error',
        message: error instanceof Error ? error.message : '加载任务失败',
      });
    }
  };

  /**
   * 加载任务结果
   */
  const loadTaskResult = async () => {
    try {
      const response = await backtestApi.getResult(taskId);
      if (response.success && response.data && response.data.result) {
        // 转换API数据格式为前端期望的格式
        const apiResult = response.data.result;
        const convertedResult: BacktestResult = {
          taskId,
          metrics: {
            totalReturn: apiResult.metrics.total_return || 0,
            annualizedReturn: apiResult.metrics.annualized_return || 0,
            maxDrawdown: apiResult.metrics.max_drawdown || 0,
            maxDrawdownDuration: apiResult.metrics.max_drawdown_duration || 0,
            sharpeRatio: apiResult.metrics.sharpe_ratio || 0,
            sortinoRatio: apiResult.metrics.sortino_ratio || 0,
            calmarRatio: apiResult.metrics.calmar_ratio || 0,
            annualizedVolatility: apiResult.metrics.annualized_volatility || 0,
            winRate: apiResult.metrics.win_rate || 0,
            totalTrades: apiResult.metrics.total_trades || 0,
            winningTrades: apiResult.metrics.total_trades 
              ? Math.round((apiResult.metrics.total_trades * (apiResult.metrics.win_rate || 0)) / 100) 
              : 0,
            losingTrades: apiResult.metrics.total_trades 
              ? apiResult.metrics.total_trades - Math.round((apiResult.metrics.total_trades * (apiResult.metrics.win_rate || 0)) / 100) 
              : 0,
            averageWin: apiResult.metrics.average_win || 0,
            averageLoss: apiResult.metrics.average_loss || 0,
            profitLossRatio: apiResult.metrics.profit_loss_ratio || 0,
            profitFactor: apiResult.metrics.profit_factor || 0,
            maxConsecutiveWins: apiResult.metrics.max_consecutive_wins || 0,
            maxConsecutiveLosses: apiResult.metrics.max_consecutive_losses || 0,
            avgHoldingPeriod: apiResult.metrics.avg_holding_period || 0,
            maxWin: apiResult.metrics.max_win || 0,
            maxLoss: apiResult.metrics.max_loss || 0,
          },
          equityCurve: apiResult.equity_curve.map((point) => ({
            time: new Date(point.timestamp * 1000).toISOString(),
            value: point.equity,
          })),
          trades: apiResult.trades.map((trade) => ({
            id: String(trade.timestamp),
            type: trade.side === 'buy' ? 'buy' : 'sell',
            symbol: 'UNKNOWN',
            price: trade.price,
            quantity: trade.quantity,
            time: new Date(trade.timestamp * 1000).toISOString(),
            pnl: trade.pnl,
            commission: trade.fee,
          })),
        };
        setResult(convertedResult);
      } else {
        throw new Error(response.error || '加载结果失败');
      }
    } catch (error) {
      addNotification({
        type: 'error',
        message: error instanceof Error ? error.message : '加载任务结果失败',
      });
    }
  };

  /**
   * 组件挂载时加载数据
   */
  useEffect(() => {
    const loadData = async () => {
      setIsLoading(true);
      try {
        await loadTask();
        await loadTaskResult();
      } finally {
        setIsLoading(false);
      }
    };
    
    if (taskId) {
      loadData();
    }
  }, [taskId]); // eslint-disable-line react-hooks/exhaustive-deps

  /**
   * 返回历史记录列表
   */
  const handleBack = () => {
    router.push('/history');
  };

  return (
    <div>
      {/* 页面头部 */}
      <PageHeader
        icon="📊"
        title={task ? `回测详情 - ${task.name}` : '回测详情'}
        action={
          <Button variant="secondary" onClick={handleBack}>
            ← 返回列表
          </Button>
        }
      />

      {/* 加载状态 */}
      {isLoading ? (
        <Card title="加载中">
          <div className="text-center py-12">
            <p className="text-gray-500">正在加载回测结果...</p>
          </div>
        </Card>
      ) : !result ? (
        // 无结果状态
        <Card title="无结果">
          <div className="text-center py-12">
            <p className="text-gray-500 mb-4">该回测任务暂无结果</p>
            <p className="text-sm text-gray-400">
              {task?.status === 'pending' && '任务正在等待执行'}
              {task?.status === 'running' && '任务正在运行中'}
              {task?.status === 'failed' && '任务执行失败'}
            </p>
          </div>
        </Card>
      ) : (
        // 结果展示
        <div className="space-y-6">
          {/* 任务信息卡片 */}
          {task && (
            <Card title="任务信息">
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div>
                  <p className="text-xs text-gray-500 mb-1">配置文件</p>
                  <p className="text-sm font-medium text-gray-900 truncate" title={task.config}>
                    {task.config}
                  </p>
                </div>
                <div>
                  <p className="text-xs text-gray-500 mb-1">数据文件</p>
                  <p className="text-sm font-medium text-gray-900 truncate" title={task.dataFile}>
                    {task.dataFile}
                  </p>
                </div>
                <div>
                  <p className="text-xs text-gray-500 mb-1">状态</p>
                  <p className="text-sm font-medium text-gray-900">
                    {task.status === 'completed' && '✅ 已完成'}
                    {task.status === 'running' && '🔄 运行中'}
                    {task.status === 'pending' && '⏳ 等待中'}
                    {task.status === 'failed' && '❌ 失败'}
                  </p>
                </div>
                <div>
                  <p className="text-xs text-gray-500 mb-1">创建时间</p>
                  <p className="text-sm font-medium text-gray-900">
                    {new Date(task.createdAt).toLocaleString('zh-CN')}
                  </p>
                </div>
              </div>
            </Card>
          )}

          {/* 性能指标卡片 */}
          <Card title="性能指标">
            <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-3">
              {/* 第一行 - 收益指标 */}
              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">总收益率</p>
                <p
                  className={`text-lg font-semibold ${
                    result.metrics.totalReturn >= 0 ? 'text-green-600' : 'text-red-600'
                  }`}
                >
                  {result.metrics.totalReturn.toFixed(2)}%
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">年化收益率</p>
                <p
                  className={`text-lg font-semibold ${
                    result.metrics.annualizedReturn >= 0 ? 'text-green-600' : 'text-red-600'
                  }`}
                >
                  {result.metrics.annualizedReturn.toFixed(2)}%
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">最大回撤</p>
                <p className="text-lg font-semibold text-red-600">
                  {result.metrics.maxDrawdown.toFixed(2)}%
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">回撤持续时间</p>
                <p className="text-lg font-semibold text-gray-900">
                  {result.metrics.maxDrawdownDuration.toFixed(1)} 天
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">夏普比率</p>
                <p className="text-lg font-semibold text-gray-900">
                  {result.metrics.sharpeRatio.toFixed(3)}
                </p>
              </div>

              {/* 第二行 - 风险调整收益 */}
              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">索提诺比率</p>
                <p className="text-lg font-semibold text-gray-900">
                  {result.metrics.sortinoRatio.toFixed(3)}
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">卡尔玛比率</p>
                <p className="text-lg font-semibold text-gray-900">
                  {result.metrics.calmarRatio.toFixed(3)}
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">年化波动率</p>
                <p className="text-lg font-semibold text-gray-900">
                  {result.metrics.annualizedVolatility.toFixed(2)}%
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">总交易次数</p>
                <p className="text-lg font-semibold text-gray-900">
                  {result.metrics.totalTrades}
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">胜率</p>
                <p className="text-lg font-semibold text-gray-900">
                  {result.metrics.winRate.toFixed(2)}%
                </p>
              </div>

              {/* 第三行 - 交易统计 */}
              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">盈利/亏损次数</p>
                <p className="text-lg font-semibold text-gray-900">
                  {result.metrics.winningTrades} / {result.metrics.losingTrades}
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">盈亏比</p>
                <p className="text-lg font-semibold text-gray-900">
                  {result.metrics.profitLossRatio.toFixed(2)}
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">盈利因子</p>
                <p className="text-lg font-semibold text-gray-900">
                  {result.metrics.profitFactor.toFixed(2)}
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">平均盈利</p>
                <p className="text-lg font-semibold text-green-600">
                  {result.metrics.averageWin.toFixed(2)}
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">平均亏损</p>
                <p className="text-lg font-semibold text-red-600">
                  {result.metrics.averageLoss.toFixed(2)}
                </p>
              </div>

              {/* 第四行 - 极值与持仓 */}
              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">最大单笔盈利</p>
                <p className="text-lg font-semibold text-green-600">
                  {result.metrics.maxWin.toFixed(2)}
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">最大单笔亏损</p>
                <p className="text-lg font-semibold text-red-600">
                  {result.metrics.maxLoss.toFixed(2)}
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">最大连胜</p>
                <p className="text-lg font-semibold text-green-600">
                  {result.metrics.maxConsecutiveWins}
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">最大连亏</p>
                <p className="text-lg font-semibold text-red-600">
                  {result.metrics.maxConsecutiveLosses}
                </p>
              </div>

              <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-1.5 font-medium">平均持仓周期</p>
                <p className="text-lg font-semibold text-gray-900">
                  {result.metrics.avgHoldingPeriod.toFixed(1)}
                </p>
              </div>
            </div>
          </Card>

          {/* 图表展示区域 */}
          <Card title="图表分析">
            <div className="space-y-6">
              <div className="p-6 bg-white rounded-lg border border-gray-200">
                <h4 className="text-base font-semibold text-gray-900 mb-4 pb-3 border-b-2 border-gray-200">
                  价格走势与交易点位
                </h4>
                <div className="h-[500px] flex items-center justify-center text-gray-400">
                  图表组件 - 待实现
                </div>
              </div>

              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div className="p-4 bg-white rounded-lg border border-gray-200">
                  <h5 className="text-sm font-semibold text-gray-900 mb-3">权益曲线</h5>
                  <div className="h-[350px] flex items-center justify-center text-gray-400">
                    图表组件 - 待实现
                  </div>
                </div>

                <div className="p-4 bg-white rounded-lg border border-gray-200">
                  <h5 className="text-sm font-semibold text-gray-900 mb-3">回撤曲线</h5>
                  <div className="h-[350px] flex items-center justify-center text-gray-400">
                    图表组件 - 待实现
                  </div>
                </div>
              </div>
            </div>
          </Card>
        </div>
      )}
    </div>
  );
}
