# Serde 使用指南

Serde 是一个用于在 Rust 数据结构和各种序列化格式之间高效、通用地进行转换的框架。

## 核心概念

- **Serialize**: 将 Rust 数据结构转换为某种格式（如 JSON、TOML）的过程。
- **Deserialize**: 将某种格式的数据转换回 Rust 数据结构的过程。
- **Derive Macros**: Serde 提供了 `#[derive(Serialize)]` 和 `#[derive(Deserialize)]` 宏，可以轻松地为您的类型实现序列化和反序列化功能。

## 设置

要开始使用 Serde，请将其和您需要的格式支持（例如 `serde_json`）添加到您的 `Cargo.toml` 文件中：

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

`derive` 功能是使用 `derive` 宏所必需的。