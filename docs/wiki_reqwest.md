# Reqwest 使用指南

Reqwest 是一个强大且易于使用的 Rust HTTP 客户端。

## 核心功能

- 发送各种类型的 HTTP 请求 (GET, POST, etc.)
- 异步 (基于 Tokio) 和阻塞 (同步) API
- JSON 支持
- 自定义请求头
- Cookie 存储

## 设置

将 `reqwest` 添加到您的 `Cargo.toml`。对于异步请求，您还需要 `tokio`。

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```

`json` 功能使得处理 JSON 响应体变得非常方便。