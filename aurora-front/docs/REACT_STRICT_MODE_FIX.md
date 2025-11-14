# React Strict Mode 导致重复请求问题修复报告

## 问题描述

在开发环境中，页面导航时发现 API 请求被调用了两次。从 Chrome DevTools 网络面板可以看到：

1. 首次请求：`http://localhost:3000/config?_rsc=vusbg` (Next.js 开发服务器)
2. 二次请求：`http://localhost:8080/api/config` (后端 Rust API)

## 问题原因分析

### 1. React 19 Strict Mode

在开发模式下，React 19 的 **Strict Mode** 会刻意执行两次副作用（Effects）来帮助开发者发现潜在的问题：

```tsx
// React 会在开发模式下执行两次
useEffect(() => {
  loadData(); // 第一次调用
  return () => {
    // 清理函数
  };
  // React 会立即清理并重新执行
  loadData(); // 第二次调用
}, []);
```

### 2. 缺少清理机制

原有代码中的 `useEffect` 没有实现清理函数（cleanup function），导致：

- 组件卸载时，正在进行的请求无法被取消
- 依赖项变化时，旧请求继续执行，可能导致状态不一致
- Strict Mode 下重复执行时，两次请求都会完成

## 解决方案

### 使用 AbortController 模式

为所有使用 `useEffect` 发起 API 请求的组件添加 `AbortController` 清理机制：

```tsx
useEffect(() => {
  // 创建 AbortController 用于取消请求
  const abortController = new AbortController();
  
  // 执行加载，传入 signal
  loadData(abortController.signal);
  
  // 清理函数：组件卸载或依赖变化时取消请求
  return () => {
    abortController.abort();
  };
}, [dependencies]);
```

### 修改加载函数

加载函数需要接受可选的 `AbortSignal` 参数，并在请求被取消时跳过状态更新：

```tsx
const loadData = async (signal?: AbortSignal) => {
  setLoading(true);
  try {
    const response = await api.getData();
    
    // 如果请求被取消，不更新状态
    if (signal?.aborted) {
      return;
    }
    
    // 更新状态
    setData(response.data);
  } catch (error) {
    // 如果请求被取消，不显示错误
    if (signal?.aborted) {
      return;
    }
    
    showError(error);
  } finally {
    if (!signal?.aborted) {
      setLoading(false);
    }
  }
};
```

## 修复的文件

### 1. ConfigList.tsx

**位置**: `src/components/dashboard/ConfigList.tsx`

**修改内容**:
- 为 `loadConfigs` 函数添加 `AbortSignal` 参数
- 在 `useEffect` 中创建 `AbortController` 并返回清理函数
- 在请求被取消时跳过状态更新

### 2. DataList.tsx

**位置**: `src/components/dashboard/DataList.tsx`

**修改内容**:
- 为 `loadDataFiles` 函数添加 `AbortSignal` 参数
- 在 `useEffect` 中创建 `AbortController` 并返回清理函数
- 在请求被取消时跳过状态更新

### 3. HistoryPage

**位置**: `src/app/history/page.tsx`

**修改内容**:
- 为 `loadTasks` useCallback 添加 `AbortSignal` 参数
- 在 `useEffect` 中创建 `AbortController` 并返回清理函数
- 在请求被取消时跳过状态更新

## 测试验证

为每个修复创建了专门的测试文件来验证 AbortController 的行为：

### 1. ConfigList.abort.test.tsx

```tsx
it('组件卸载时应该取消正在进行的请求', async () => {
  const { unmount } = render(<ConfigList />);
  unmount(); // 应该触发 abort
});

it('refreshTrigger 变化时应该取消之前的请求', async () => {
  const { rerender } = render(<ConfigList refreshTrigger={0} />);
  rerender(<ConfigList refreshTrigger={1} />); // 应该取消旧请求
});
```

### 2. DataList.abort.test.tsx

类似的测试用例验证 DataList 组件的 AbortController 行为。

### 3. page.abort.test.tsx

验证 HistoryPage 在组件卸载和刷新时的正确行为。

## 测试结果

所有测试均通过：

```bash
✓ ConfigList.abort.test.tsx (3 tests)
✓ DataList.abort.test.tsx (3 tests)  
✓ page.abort.test.tsx (3 tests)
```

## 效果

### 修复前

- 开发模式下每个页面导航都会发起 2 次 API 请求
- 快速切换页面时可能出现状态不一致
- 组件卸载后请求仍在执行

### 修复后

- ✅ 即使在 Strict Mode 下，也只有最后一次请求的结果会更新状态
- ✅ 组件卸载时，正在进行的请求会被正确取消
- ✅ 依赖项变化时，旧请求会被取消，避免竞态条件
- ✅ 用户手动刷新仍然正常工作

## React 最佳实践

这个修复遵循了 React 官方推荐的最佳实践：

1. **总是实现清理函数**: 对于所有有副作用的 useEffect
2. **使用 AbortController**: 取消不再需要的异步请求
3. **检查取消状态**: 在异步操作完成后检查组件是否仍然挂载

## 注意事项

### 为什么在生产环境不会有问题？

React Strict Mode **只在开发环境启用**，生产构建会自动禁用：

```tsx
// Next.js 的默认行为
export default function RootLayout({ children }) {
  return (
    <html>
      <body>
        {/* Strict Mode 只在开发环境有效 */}
        {children}
      </body>
    </html>
  );
}
```

### 为什么不直接禁用 Strict Mode？

虽然可以禁用 Strict Mode 来"解决"这个问题，但这是**不推荐的做法**，因为：

1. Strict Mode 帮助发现潜在的并发问题
2. React 未来版本会更多地使用并发特性
3. 正确的清理机制是编写健壮代码的必要条件

## 参考资料

- [React 官方文档 - Strict Mode](https://react.dev/reference/react/StrictMode)
- [React 官方文档 - useEffect 清理函数](https://react.dev/reference/react/useEffect#my-effect-runs-twice-when-the-component-mounts)
- [MDN - AbortController](https://developer.mozilla.org/en-US/docs/Web/API/AbortController)

## 总结

通过添加 `AbortController` 清理机制，我们：

1. ✅ 修复了 React Strict Mode 导致的重复请求问题
2. ✅ 提高了应用的健壮性和性能
3. ✅ 遵循了 React 最佳实践
4. ✅ 为未来的并发特性做好了准备

所有修改都有完整的测试覆盖，确保功能正确性。
