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
import { PageHeader, Button, Card, Tabs } from '@/components/ui';
import {
  EquityCurveChart,
  DrawdownChart,
  MonthlyReturnsHeatmap,
  ReturnsDistribution,
  TradesPnLChart,
  RollingMetricsChart,
} from '@/components/charts';
import { BacktestResult, BacktestTask, DrawdownPoint, MonthlyReturn } from '@/types';
import { backtestApi } from '@/lib/api';
import { useNotificationStore } from '@/lib/store/notificationStore';

/**
 * 计算回撤序列
 * 
 * 从权益曲线计算每个时间点的回撤百分比
 */
function calculateDrawdownSeries(equityCurve: { timestamp: number; equity: number }[]): DrawdownPoint[] {
  if (equityCurve.length === 0) return [];

  const drawdownSeries: DrawdownPoint[] = [];
  let peak = equityCurve[0].equity;

  equityCurve.forEach((point) => {
    // 更新最高点
    if (point.equity > peak) {
      peak = point.equity;
    }

    // 计算回撤百分比
    const drawdown = (point.equity - peak) / peak;

    drawdownSeries.push({
      time: new Date(point.timestamp).toISOString(),
      drawdown,
    });
  });

  return drawdownSeries;
}

/**
 * 计算月度收益
 * 
 * 从权益曲线计算每个月的收益率
 */
function calculateMonthlyReturns(equityCurve: { timestamp: number; equity: number }[]): MonthlyReturn[] {
  if (equityCurve.length === 0) return [];

  // 按月分组
  const monthlyData = new Map<string, { start: number; end: number }>();

  equityCurve.forEach((point) => {
    const date = new Date(point.timestamp);
    const yearMonth = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}`;

    if (!monthlyData.has(yearMonth)) {
      monthlyData.set(yearMonth, { start: point.equity, end: point.equity });
    } else {
      const data = monthlyData.get(yearMonth)!;
      data.end = point.equity;
    }
  });

  // 计算每月收益率
  const monthlyReturns: MonthlyReturn[] = [];
  monthlyData.forEach((data, yearMonth) => {
    const [year, month] = yearMonth.split('-').map(Number);
    const returnPct = ((data.end - data.start) / data.start) * 100;

    monthlyReturns.push({
      year,
      month,
      return: returnPct,
    });
  });

  return monthlyReturns.sort((a, b) => {
    if (a.year !== b.year) return a.year - b.year;
    return a.month - b.month;
  });
}

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
          equityCurve: apiResult.equity_curve
            .slice()
            .sort((a, b) => a.timestamp - b.timestamp)
            .map((point) => ({
              time: Math.floor(point.timestamp / 1000), // 转换为 Unix 秒级时间戳
              value: point.equity,
            })),
          trades: apiResult.trades
            .slice()
            .sort((a, b) => a.timestamp - b.timestamp)
            .map((trade) => ({
            id: String(trade.timestamp),
            type: trade.side === 'buy' ? 'buy' : 'sell',
            symbol: 'UNKNOWN',
            price: trade.price,
            quantity: trade.quantity,
            time: Math.floor(trade.timestamp / 1000), // 转换为 Unix 秒级时间戳
            pnl: trade.pnl,
            commission: trade.fee,
          })),
          // 计算回撤序列（基于权益曲线）
          drawdownSeries: calculateDrawdownSeries(
            [...apiResult.equity_curve].sort((a, b) => a.timestamp - b.timestamp)
          ),
          // 计算月度收益
          monthlyReturns: calculateMonthlyReturns(apiResult.equity_curve),
          // 滚动指标数据（暂时为空，后续可从后端获取）
          rollingMetrics: [],
          // 收益分布数据（前端计算）
          returnsDistribution: undefined,
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

          {/* 图表分析 - 分Tab展示 */}
          <Card title="图表分析">
            <Tabs
              tabs={[
                {
                  id: 'overview',
                  label: '综合概览',
                  icon: '📈',
                  content: (
                    <div className="space-y-6">
                      {/* 累计净值曲线 */}
                      <div>
                        <h4 className="text-base font-semibold text-gray-900 mb-4">累计净值曲线</h4>
                        <EquityCurveChart data={result.equityCurve} />
                      </div>

                      {/* 回撤曲线与月度收益热力图 */}
                      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                        <div>
                          <h5 className="text-sm font-semibold text-gray-900 mb-3">回撤曲线（潜水图）</h5>
                          {result.drawdownSeries && result.drawdownSeries.length > 0 ? (
                            <DrawdownChart data={result.drawdownSeries} />
                          ) : (
                            <div className="h-80 flex items-center justify-center text-gray-400">
                              暂无回撤数据
                            </div>
                          )}
                        </div>

                        <div>
                          <h5 className="text-sm font-semibold text-gray-900 mb-3">收益分布直方图</h5>
                          <ReturnsDistribution equityCurve={result.equityCurve} height={350} />
                        </div>
                      </div>

                      {/* 月度收益热力图 */}
                      {result.monthlyReturns && result.monthlyReturns.length > 0 && (
                        <div>
                          <h5 className="text-sm font-semibold text-gray-900 mb-3">月度收益热力图</h5>
                          <MonthlyReturnsHeatmap data={result.monthlyReturns} height={400} />
                        </div>
                      )}
                    </div>
                  ),
                },
                {
                  id: 'trades',
                  label: '交易细节',
                  icon: '💹',
                  content: (
                    <div className="space-y-6">
                      {/* 交易盈亏分布 */}
                      <div>
                        <h4 className="text-base font-semibold text-gray-900 mb-4">交易盈亏分布</h4>
                        <TradesPnLChart trades={result.trades} />
                      </div>

                      {/* 交易列表 */}
                      <div>
                        <h4 className="text-base font-semibold text-gray-900 mb-4">交易记录</h4>
                        <div className="overflow-x-auto">
                          <table className="w-full text-sm">
                            <thead className="bg-gray-50 border-b border-gray-200">
                              <tr>
                                <th className="px-4 py-2 text-left font-semibold text-gray-700">时间</th>
                                <th className="px-4 py-2 text-left font-semibold text-gray-700">类型</th>
                                <th className="px-4 py-2 text-right font-semibold text-gray-700">价格</th>
                                <th className="px-4 py-2 text-right font-semibold text-gray-700">数量</th>
                                <th className="px-4 py-2 text-right font-semibold text-gray-700">盈亏</th>
                                <th className="px-4 py-2 text-right font-semibold text-gray-700">手续费</th>
                              </tr>
                            </thead>
                            <tbody>
                              {result.trades.slice(0, 100).map((trade) => (
                                <tr key={trade.id} className="border-b border-gray-100 hover:bg-gray-50">
                                  <td className="px-4 py-2 text-gray-900">
                                    {new Date(trade.time).toLocaleString('zh-CN')}
                                  </td>
                                  <td className="px-4 py-2">
                                    <span
                                      className={`px-2 py-1 rounded text-xs font-medium ${
                                        trade.type === 'buy'
                                          ? 'bg-green-100 text-green-700'
                                          : 'bg-red-100 text-red-700'
                                      }`}
                                    >
                                      {trade.type === 'buy' ? '买入' : '卖出'}
                                    </span>
                                  </td>
                                  <td className="px-4 py-2 text-right text-gray-900">
                                    {trade.price.toFixed(2)}
                                  </td>
                                  <td className="px-4 py-2 text-right text-gray-900">
                                    {trade.quantity.toFixed(4)}
                                  </td>
                                  <td
                                    className={`px-4 py-2 text-right font-medium ${
                                      (trade.pnl || 0) >= 0 ? 'text-green-600' : 'text-red-600'
                                    }`}
                                  >
                                    {trade.pnl !== undefined ? trade.pnl.toFixed(2) : '-'}
                                  </td>
                                  <td className="px-4 py-2 text-right text-gray-600">
                                    {trade.commission !== undefined ? trade.commission.toFixed(2) : '-'}
                                  </td>
                                </tr>
                              ))}
                            </tbody>
                          </table>
                          {result.trades.length > 100 && (
                            <p className="text-center text-sm text-gray-500 mt-4">
                              仅显示前 100 笔交易，共 {result.trades.length} 笔
                            </p>
                          )}
                        </div>
                      </div>
                    </div>
                  ),
                },
                {
                  id: 'risk',
                  label: '风险分析',
                  icon: '⚠️',
                  content: (
                    <div className="space-y-6">
                      {/* 滚动指标 */}
                      {result.rollingMetrics && result.rollingMetrics.length > 0 ? (
                        <div>
                          <h4 className="text-base font-semibold text-gray-900 mb-4">滚动波动率与夏普比率</h4>
                          <RollingMetricsChart data={result.rollingMetrics} />
                        </div>
                      ) : (
                        <div className="text-center py-12 text-gray-400">
                          滚动指标数据暂未计算，可在后续版本中添加
                        </div>
                      )}

                      {/* 风险指标卡片 */}
                      <div>
                        <h4 className="text-base font-semibold text-gray-900 mb-4">风险指标</h4>
                        <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
                          <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
                            <p className="text-sm text-gray-600 mb-1">最大回撤</p>
                            <p className="text-2xl font-bold text-red-600">
                              {result.metrics.maxDrawdown.toFixed(2)}%
                            </p>
                          </div>
                          <div className="p-4 bg-orange-50 border border-orange-200 rounded-lg">
                            <p className="text-sm text-gray-600 mb-1">回撤持续时间</p>
                            <p className="text-2xl font-bold text-orange-600">
                              {result.metrics.maxDrawdownDuration.toFixed(0)} 天
                            </p>
                          </div>
                          <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
                            <p className="text-sm text-gray-600 mb-1">年化波动率</p>
                            <p className="text-2xl font-bold text-yellow-700">
                              {result.metrics.annualizedVolatility.toFixed(2)}%
                            </p>
                          </div>
                        </div>
                      </div>
                    </div>
                  ),
                },
              ]}
              defaultActiveId="overview"
            />
          </Card>
        </div>
      )}
    </div>
  );
}
