import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  
  // API 代理配置 - 开发环境下代理到 Rust 后端
  async rewrites() {
    // 只在开发环境启用代理
    if (process.env.NODE_ENV === 'development') {
      return [
        {
          source: '/api/:path*',
          destination: 'http://localhost:8080/api/:path*',
        },
      ];
    }
    return [];
  },
};

export default nextConfig;
