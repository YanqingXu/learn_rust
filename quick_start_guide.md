# Rust 快速入门指南

## 立即开始

### 1. 安装 Rust（5分钟）

#### Windows 安装步骤
1. 访问 [rustup.rs](https://rustup.rs/)
2. 下载并运行 `rustup-init.exe`
3. 按照提示完成安装
4. 重启终端或命令提示符

#### 验证安装
```bash
# 检查 Rust 版本
rustc --version

# 检查 Cargo 版本
cargo --version

# 检查 rustup 版本
rustup --version
```

### 2. 配置开发环境（10分钟）

#### VS Code 配置（推荐）
1. 安装 VS Code
2. 安装扩展：
   - `rust-analyzer`（官方推荐）
   - `CodeLLDB`（调试支持）
   - `crates`（依赖管理）

#### 其他编辑器选择
- **IntelliJ IDEA/CLion**：安装 Rust 插件
- **Vim/Neovim**：配置 rust-analyzer LSP
- **Emacs**：使用 rustic 模式

### 3. 创建第一个项目（5分钟）

```bash
# 创建新项目
cargo new hello_rust
cd hello_rust

# 查看项目结构
tree
# 或者
dir /s  # Windows
ls -la  # Linux/Mac
```

项目结构：
```
hello_rust/
├── Cargo.toml    # 项目配置文件
└── src/
    └── main.rs   # 主源文件
```

### 4. 编写第一个程序（5分钟）

编辑 `src/main.rs`：
```rust
fn main() {
    println!("Hello, Rust!");
    
    // 变量声明
    let name = "程序员";
    let age = 25;
    
    println!("你好，{}！你今年{}岁。", name, age);
    
    // 可变变量
    let mut count = 0;
    count += 1;
    println!("计数：{}", count);
}
```

### 5. 运行程序（2分钟）

```bash
# 编译并运行
cargo run

# 仅编译
cargo build

# 发布版本编译
cargo build --release

# 检查代码（不生成可执行文件）
cargo check
```

## 第一天学习计划

### 上午（2小时）：基础语法

#### 1. 变量与数据类型（30分钟）

创建 `src/variables.rs`：
```rust
fn main() {
    // 不可变变量
    let x = 5;
    println!("x 的值是：{}", x);
    
    // 可变变量
    let mut y = 10;
    println!("y 的初始值：{}", y);
    y = 15;
    println!("y 的新值：{}", y);
    
    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("最大分数：{}", MAX_POINTS);
    
    // 变量遮蔽
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("z 的最终值：{}", z);
    
    // 数据类型
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = '🦀';
    
    println!("整数：{}，浮点数：{}，布尔值：{}，字符：{}", 
             integer, float, boolean, character);
}
```

运行：`cargo run --bin variables`

#### 2. 函数（30分钟）

创建 `src/functions.rs`：
```rust
fn main() {
    println!("主函数开始");
    
    greet();
    greet_person("Alice");
    
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    let (sum, product) = calculate(4, 6);
    println!("4 + 6 = {}，4 × 6 = {}", sum, product);
}

// 无参数无返回值函数
fn greet() {
    println!("Hello, World!");
}

// 有参数无返回值函数
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// 有参数有返回值函数
fn add(a: i32, b: i32) -> i32 {
    a + b  // 注意：没有分号，这是表达式
}

// 返回多个值
fn calculate(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}
```

#### 3. 控制流（60分钟）

创建 `src/control_flow.rs`：
```rust
fn main() {
    // if 表达式
    let number = 6;
    
    if number % 4 == 0 {
        println!("数字能被 4 整除");
    } else if number % 3 == 0 {
        println!("数字能被 3 整除");
    } else if number % 2 == 0 {
        println!("数字能被 2 整除");
    } else {
        println!("数字不能被 4、3 或 2 整除");
    }
    
    // if 作为表达式
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number 的值是：{}", number);
    
    // loop 循环
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop 结果：{}", result);
    
    // while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("发射！");
    
    // for 循环
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("值是：{}", element);
    }
    
    // 范围循环
    for number in 1..4 {
        println!("{}!", number);
    }
    println!("发射！");
}
```

### 下午（2小时）：所有权基础

#### 1. 所有权概念（60分钟）

创建 `src/ownership.rs`：
```rust
fn main() {
    // 字符串字面值（存储在栈上）
    let s1 = "hello";
    println!("s1: {}", s1);
    
    // String 类型（存储在堆上）
    let s2 = String::from("hello");
    println!("s2: {}", s2);
    
    // 移动（Move）
    let s3 = s2;
    // println!("s2: {}", s2); // 这行会报错，因为 s2 已经被移动
    println!("s3: {}", s3);
    
    // 克隆（Clone）
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);
    
    // 函数调用和所有权
    let s5 = String::from("hello");
    takes_ownership(s5);
    // println!("s5: {}", s5); // 这行会报错
    
    let x = 5;
    makes_copy(x);
    println!("x: {}", x); // 这行正常，因为 i32 实现了 Copy trait
    
    // 返回值和所有权
    let s6 = gives_ownership();
    println!("s6: {}", s6);
    
    let s7 = String::from("hello");
    let s8 = takes_and_gives_back(s7);
    // println!("s7: {}", s7); // 这行会报错
    println!("s8: {}", s8);
}

fn takes_ownership(some_string: String) {
    println!("接收到：{}", some_string);
} // some_string 在这里被丢弃

fn makes_copy(some_integer: i32) {
    println!("接收到：{}", some_integer);
} // some_integer 在这里被丢弃，但没有特殊操作

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 返回 some_string 并移动给调用函数
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回 a_string 并移动给调用函数
}
```

#### 2. 引用和借用（60分钟）

创建 `src/references.rs`：
```rust
fn main() {
    let s1 = String::from("hello");
    
    // 不可变引用
    let len = calculate_length(&s1);
    println!("'{}' 的长度是 {}。", s1, len);
    
    // 可变引用
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("修改后：{}", s2);
    
    // 引用规则演示
    let mut s3 = String::from("hello");
    
    // 多个不可变引用是允许的
    let r1 = &s3;
    let r2 = &s3;
    println!("{} 和 {}", r1, r2);
    
    // 不可变引用的作用域结束后，可以创建可变引用
    let r3 = &mut s3;
    println!("{}", r3);
    
    // 字符串切片
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
    
    // 使用字符串切片的函数
    let word = first_word(&s);
    println!("第一个单词：{}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s 是引用，所以不会获取所有权

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

### 配置 Cargo.toml 支持多个二进制文件

编辑 `Cargo.toml`：
```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "main"
path = "src/main.rs"

[[bin]]
name = "variables"
path = "src/variables.rs"

[[bin]]
name = "functions"
path = "src/functions.rs"

[[bin]]
name = "control_flow"
path = "src/control_flow.rs"

[[bin]]
name = "ownership"
path = "src/ownership.rs"

[[bin]]
name = "references"
path = "src/references.rs"
```

## 常用命令速查

```bash
# 项目管理
cargo new project_name          # 创建新项目
cargo init                       # 在当前目录初始化项目
cargo build                      # 编译项目
cargo run                        # 编译并运行
cargo run --bin binary_name      # 运行指定二进制文件
cargo check                      # 检查代码（快速）
cargo test                       # 运行测试
cargo doc --open                 # 生成并打开文档

# 依赖管理
cargo add serde                  # 添加依赖
cargo update                     # 更新依赖
cargo tree                       # 查看依赖树

# 代码格式化和检查
cargo fmt                        # 格式化代码
cargo clippy                     # 代码检查和建议

# 发布
cargo build --release            # 发布版本编译
cargo publish                    # 发布到 crates.io
```

## 学习资源快速链接

- **官方教程**：https://doc.rust-lang.org/book/
- **Rust by Example**：https://doc.rust-lang.org/rust-by-example/
- **在线练习**：https://github.com/rust-lang/rustlings
- **在线编译器**：https://play.rust-lang.org/
- **API 文档**：https://doc.rust-lang.org/std/

## 遇到问题时

1. **编译错误**：仔细阅读错误信息，Rust 编译器的提示非常详细
2. **所有权问题**：回顾所有权规则，多练习引用的使用
3. **语法疑问**：查阅官方文档或 Rust by Example
4. **社区求助**：
   - [Rust 官方论坛](https://users.rust-lang.org/)
   - [Stack Overflow](https://stackoverflow.com/questions/tagged/rust)
   - [Reddit r/rust](https://www.reddit.com/r/rust/)

## 下一步

完成第一天的学习后，继续按照 `rust_learning_plan.md` 中的计划进行系统学习。记住：

- **多动手**：理论结合实践
- **多思考**：理解概念背后的原理
- **多练习**：通过 rustlings 等工具强化
- **不急躁**：Rust 学习曲线较陡，需要耐心

祝你 Rust 学习之旅愉快！🦀