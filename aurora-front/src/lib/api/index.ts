// Copyright 2025 blingbling21
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/**
 * API 服务统一导出
 */

// 导出 API 客户端基础设施
export * from './client';

// 导出各个服务模块
export * from './config';
export * from './data';
export * from './backtest';
export * from './dashboard';

// 统一 API 对象
import { configApi } from './config';
import { dataApi } from './data';
import { backtestApi } from './backtest';
import { DashboardService } from './dashboard';

export const api = {
  config: configApi,
  data: dataApi,
  backtest: backtestApi,
  dashboard: DashboardService,
};
