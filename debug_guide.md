# Rust 项目调试指南

## 🚀 快速开始

### 1. 环境要求
- **VS Code**: 安装最新版本
- **Rust 扩展**: 安装 `rust-analyzer` 扩展
- **调试器**: 安装 `CodeLLDB` 扩展（用于调试）
- **Rust 工具链**: 确保已安装 `rustc`, `cargo`, `clippy`, `rustfmt`

### 2. 调试配置文件说明

#### `.vscode/launch.json` - 调试配置
包含以下预配置的调试选项：
- 🦀 **Debug Main Program** - 调试主程序
- 🧮 **Debug Calculator** - 调试计算器项目
- 📦 **Debug Variables** - 调试变量学习模块
- 🔧 **Debug Functions** - 调试函数学习模块
- 🏠 **Debug Ownership** - 调试所有权学习模块
- 🧪 **Debug Tests** - 调试测试

#### `.vscode/tasks.json` - 构建任务
包含以下任务：
- `rust: cargo build main` - 构建主程序
- `rust: cargo run main` - 运行主程序
- `rust: cargo test main` - 测试主程序
- `rust: cargo check all` - 检查所有代码
- `rust: cargo clippy` - 代码质量检查

#### `.vscode/settings.json` - 工作区设置
配置了：
- Rust Analyzer 设置
- 代码格式化
- 编辑器优化
- 调试环境

## 🔧 如何调试 main 程序

### 方法1: 使用调试面板
1. 打开 VS Code
2. 按 `Ctrl+Shift+D` 打开调试面板
3. 在下拉菜单中选择 "🦀 Debug Main Program"
4. 点击绿色播放按钮或按 `F5` 开始调试

### 方法2: 使用快捷键
1. 打开 `src/main.rs` 文件
2. 设置断点（点击行号左侧或按 `F9`）
3. 按 `F5` 开始调试
4. 程序会在断点处暂停

### 方法3: 使用命令面板
1. 按 `Ctrl+Shift+P` 打开命令面板
2. 输入 "Debug: Start Debugging"
3. 选择 "🦀 Debug Main Program"

## 🛠️ 调试功能详解

### 断点设置
- **普通断点**: 点击行号左侧
- **条件断点**: 右键点击断点 → "Edit Breakpoint" → 设置条件
- **日志断点**: 右键点击断点 → "Edit Breakpoint" → 设置日志消息

### 调试控制
- `F5` - 继续执行
- `F10` - 单步跳过（Step Over）
- `F11` - 单步进入（Step Into）
- `Shift+F11` - 单步跳出（Step Out）
- `Ctrl+Shift+F5` - 重新启动调试
- `Shift+F5` - 停止调试

### 调试面板功能
1. **变量窗口**: 查看当前作用域的变量值
2. **监视窗口**: 添加自定义表达式监视
3. **调用堆栈**: 查看函数调用链
4. **断点窗口**: 管理所有断点

## 📋 调试最佳实践

### 1. 设置环境变量
项目已配置以下环境变量：
- `RUST_LOG=debug` - 启用详细日志
- `RUST_BACKTRACE=1` - 显示错误堆栈跟踪

### 2. 使用 println! 调试
```rust
// 基本输出
println!("变量值: {}", variable);

// 调试格式输出
println!("调试信息: {:?}", complex_variable);

// 美化调试输出
println!("详细调试: {:#?}", complex_variable);
```

### 3. 使用 dbg! 宏
```rust
// 快速调试，会打印变量名和值
let result = dbg!(some_calculation());

// 调试表达式
dbg!(x + y);
```

### 4. 条件编译调试代码
```rust
#[cfg(debug_assertions)]
println!("这只在调试模式下运行");
```

## 🧪 测试调试

### 运行单个测试
```bash
cargo test test_function_name
```

### 调试测试
1. 在测试函数中设置断点
2. 选择 "🧪 Debug Tests" 配置
3. 开始调试

### 测试输出
```bash
# 显示测试输出
cargo test -- --nocapture

# 显示被忽略的测试
cargo test -- --ignored
```

## 🔍 性能调试

### 使用 Criterion 基准测试
```bash
# 运行基准测试
cargo bench

# 生成性能报告
cargo bench -- --output-format html
```

### 内存使用分析
```bash
# 使用 Valgrind（Linux/macOS）
valgrind --tool=memcheck cargo run --bin main

# 使用 heaptrack（Linux）
heaptrack cargo run --bin main
```

## 🚨 常见问题解决

### 1. 调试器无法启动
- 确保安装了 CodeLLDB 扩展
- 检查 Rust 工具链是否正确安装
- 重启 VS Code

### 2. 断点不生效
- 确保在 Debug 模式下编译（默认 `cargo build`）
- 检查代码是否被优化掉
- 确保断点设置在可执行的代码行

### 3. 变量值显示 "optimized out"
- 在 `Cargo.toml` 中添加调试配置：
```toml
[profile.dev]
opt-level = 0
debug = true
```

### 4. 调试信息不完整
- 确保编译时包含调试信息：
```bash
cargo build --bin main
```

## 📚 学习建议

### 调试学习路径
1. **基础调试**: 从 `variables.rs` 开始，学习基本的断点和变量查看
2. **函数调试**: 在 `functions.rs` 中练习单步调试和调用堆栈
3. **所有权调试**: 在 `ownership.rs` 中观察所有权转移
4. **项目调试**: 在 `calculator.rs` 中进行复杂逻辑调试

### 调试技巧
- 从简单的 `println!` 开始
- 逐步学习使用断点
- 练习查看变量和表达式
- 学习分析调用堆栈
- 掌握条件断点的使用

## 🔗 相关资源

- [Rust 官方调试指南](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [VS Code Rust 扩展文档](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [CodeLLDB 调试器文档](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
- [Cargo 官方文档](https://doc.rust-lang.org/cargo/)

---

💡 **提示**: 调试是学习 Rust 的重要技能，多练习使用这些工具将大大提高你的开发效率！