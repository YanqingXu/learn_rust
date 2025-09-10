# 项目结构说明

这个 Rust 学习项目为你提供了一个完整的学习环境，包含了从基础到高级的所有重要概念。

## 📁 目录结构

```
learn_rust/
├── Cargo.toml                    # 项目配置文件
├── README.md                      # 项目概览
├── rust_learning_plan.md          # 详细学习计划
├── learning_progress.md           # 学习进度跟踪
├── quick_start_guide.md           # 快速入门指南
├── project_structure.md           # 项目结构说明（当前文件）
└── src/
    ├── main.rs                    # 主程序入口
    ├── lib.rs                     # 库文件（包含学习工具）
    ├── basics/                    # 基础语法学习
    │   ├── variables.rs           # 变量和可变性
    │   ├── functions.rs           # 函数定义和调用
    │   ├── control_flow.rs        # 控制流（待创建）
    │   └── data_types.rs          # 数据类型（待创建）
    ├── ownership/                 # 所有权系统
    │   ├── ownership.rs           # 所有权基础
    │   ├── references.rs          # 引用和借用（待创建）
    │   └── slices.rs              # 切片（待创建）
    ├── structs_enums/             # 结构体和枚举（待创建）
    │   ├── structs.rs
    │   ├── enums.rs
    │   └── pattern_matching.rs
    ├── error_handling/            # 错误处理（待创建）
    │   ├── error_handling.rs
    │   └── result_option.rs
    ├── generics_traits/           # 泛型和特征（待创建）
    │   ├── generics.rs
    │   ├── traits.rs
    │   └── lifetimes.rs
    ├── collections/               # 集合类型（待创建）
    │   ├── collections.rs
    │   ├── vectors.rs
    │   ├── strings.rs
    │   └── hashmaps.rs
    ├── functional/                # 函数式编程（待创建）
    │   ├── closures.rs
    │   └── iterators.rs
    ├── concurrency/               # 并发编程（待创建）
    │   ├── threads.rs
    │   ├── channels.rs
    │   └── shared_state.rs
    └── projects/                  # 实践项目
        ├── calculator.rs          # 简单计算器
        ├── todo_app.rs            # 待办事项管理器（待创建）
        └── file_search.rs         # 文件搜索工具（待创建）
```

## 🚀 如何使用

### 1. 运行主程序
```bash
cargo run
```
这会显示所有可用的学习模块和使用说明。

### 2. 运行特定的学习模块
```bash
# 基础语法
cargo run --bin variables
cargo run --bin functions

# 所有权系统
cargo run --bin ownership

# 实践项目
cargo run --bin calculator
```

### 3. 运行测试
```bash
# 运行所有测试
cargo test

# 运行特定模块的测试
cargo test variables
cargo test calculator
```

### 4. 检查代码
```bash
# 快速检查（不生成可执行文件）
cargo check

# 代码格式化
cargo fmt

# 代码检查和建议
cargo clippy
```

## 📚 学习路径建议

### 第一周：基础语法
1. **变量和可变性** - `cargo run --bin variables`
2. **函数** - `cargo run --bin functions`
3. **控制流** - `cargo run --bin control_flow`（需要创建）
4. **数据类型** - `cargo run --bin data_types`（需要创建）

### 第二周：所有权系统
1. **所有权基础** - `cargo run --bin ownership`
2. **引用和借用** - `cargo run --bin references`（需要创建）
3. **切片** - `cargo run --bin slices`（需要创建）

### 第三周：复合数据类型
1. **结构体** - `cargo run --bin structs`（需要创建）
2. **枚举** - `cargo run --bin enums`（需要创建）
3. **模式匹配** - `cargo run --bin pattern_matching`（需要创建）

### 实践项目
- **计算器** - `cargo run --bin calculator`（已完成）
- **待办事项管理器** - `cargo run --bin todo_app`（需要创建）
- **文件搜索工具** - `cargo run --bin file_search`（需要创建）

## 🛠️ 开发工具

### 已配置的依赖
- `serde` - 序列化/反序列化
- `serde_json` - JSON 处理
- `rand` - 随机数生成
- `clap` - 命令行参数解析
- `tokio` - 异步运行时
- `reqwest` - HTTP 客户端

### 开发依赖
- `criterion` - 性能基准测试

## 📊 学习进度跟踪

项目包含了一个内置的进度跟踪系统，你可以在代码中使用：

```rust
use learn_rust::progress::{ProgressTracker, Topic, Status};

let mut tracker = ProgressTracker::new();
tracker.update_status(Topic::Variables, Status::Completed);
tracker.show_progress();
```

## 🔧 扩展项目

### 添加新的学习模块
1. 在相应目录下创建 `.rs` 文件
2. 在 `Cargo.toml` 中添加对应的 `[[bin]]` 配置
3. 编写学习代码和测试

### 示例：添加新模块
```toml
# 在 Cargo.toml 中添加
[[bin]]
name = "new_topic"
path = "src/category/new_topic.rs"
```

## 💡 学习建议

### 最佳实践
1. **按顺序学习**：从基础语法开始，逐步深入
2. **动手实践**：每学完一个概念就运行相应的代码
3. **阅读测试**：测试代码展示了如何使用各种特性
4. **修改代码**：尝试修改示例代码，观察结果
5. **写笔记**：在 `learning_progress.md` 中记录学习心得

### 遇到问题时
1. **查看编译错误**：Rust 编译器的错误信息非常详细
2. **运行测试**：`cargo test` 可以验证代码正确性
3. **查阅文档**：使用 `cargo doc --open` 生成并查看文档
4. **参考资源**：查看 `rust_learning_plan.md` 中的学习资源

## 🎯 学习目标

完成这个项目后，你将能够：
- 理解 Rust 的核心概念（所有权、借用、生命周期）
- 编写安全、高效的 Rust 代码
- 使用 Rust 的标准库和生态系统
- 开发实际的 Rust 应用程序
- 进行错误处理和测试
- 理解并发编程概念

## 📞 获取帮助

- **官方文档**：https://doc.rust-lang.org/book/
- **Rust by Example**：https://doc.rust-lang.org/rust-by-example/
- **社区论坛**：https://users.rust-lang.org/
- **练习题**：https://github.com/rust-lang/rustlings

---

**开始你的 Rust 学习之旅吧！** 🦀

记住：Rust 的学习曲线可能比较陡峭，但一旦掌握，你将获得强大的系统编程能力。坚持练习，不断实践！