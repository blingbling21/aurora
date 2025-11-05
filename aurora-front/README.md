# Aurora Front - 量化交易回测平台前端

基于 Next.js 16 的现代化量化交易回测平台前端应用。

## 架构说明

Aurora 采用前后端分离架构：

- **前端 (aurora-front)**: Next.js + React - 运行在 localhost:3000
- **后端 (aurora-web)**: Rust + Axum - 运行在 localhost:8080

⚠️ **重要**: 前端通过代理访问后端 API，需要同时启动前后端服务。

## 快速开始

### 1. 启动后端服务器

```bash
cd ../aurora-web
cargo run
```

后端将在 `http://localhost:8080` 启动

### 2. 启动前端开发服务器

```bash
npm install  # 首次运行需要安装依赖
npm run dev
```

前端将在 `http://localhost:3000` 启动

### 3. 访问应用

打开浏览器访问 [http://localhost:3000](http://localhost:3000)

## API 配置

前端通过环境变量和 Next.js 代理访问后端 API：

- **开发环境**: 使用 Next.js rewrites 代理到 `http://localhost:8080/api`
- **生产环境**: 通过 `NEXT_PUBLIC_API_BASE_URL` 环境变量配置

详细配置说明请参考: [API_CONFIGURATION.md](./docs/API_CONFIGURATION.md)

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/app/building-your-application/optimizing/fonts) to automatically optimize and load [Geist](https://vercel.com/font), a new font family for Vercel.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.
