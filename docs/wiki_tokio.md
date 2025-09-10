# Tokio 使用指南

Tokio 是一个用于使用 Rust 编程语言编写可靠、异步应用程序的运行时。它专为 I/O 密集型应用设计，例如网络服务器和客户端，能够以最小的开销处理成千上万的并发连接。

## 目录

1. [为什么选择 Tokio？](#为什么选择-tokio)
2. [核心组件](#核心组件)
3. [何时不应该使用 Tokio](#何时不应该使用-tokio)
4. [async/await 入门](#asyncawait-入门)
5. [设置和配置](#设置和配置)
6. [Tokio 运行时详解](#tokio-运行时详解)
7. [异步 I/O 操作](#异步-io-操作)
8. [任务和并发](#任务和并发)
9. [网络编程](#网络编程)
10. [定时器和延迟](#定时器和延迟)
11. [同步原语](#同步原语)
12. [错误处理](#错误处理)
13. [性能优化](#性能优化)
14. [最佳实践](#最佳实践)
15. [常见问题和解决方案](#常见问题和解决方案)
16. [生态系统](#生态系统)
17. [实际项目示例](#实际项目示例)

## 为什么选择 Tokio？

### 高性能

Tokio 构建在 Rust 之上，并利用 `async/await` 语法，提供了极高的性能和可伸缩性。它的多线程、工作窃取的调度器能高效地在多个核心上执行任务。

**性能特点：**
- 零成本抽象：Tokio 的抽象不会带来运行时开销
- 工作窃取调度器：自动平衡负载
- 高效的内存使用：最小化内存分配
- 可扩展到数百万并发连接

### 可靠性

借助 Rust 的所有权和类型系统，Tokio 帮助你编写内存安全和线程安全的代码，从根本上消除了一整类常见的并发 bug。

**安全保证：**
- 内存安全：无空指针、缓冲区溢出
- 线程安全：编译时检查数据竞争
- 类型安全：强类型系统防止误用
- 无未定义行为

### 易于使用

`async/await` 语法大大降低了编写异步代码的复杂性。Tokio 提供了与标准库类似的 API，使得上手和迁移变得简单。

**开发体验：**
- 直观的 API 设计
- 丰富的文档和示例
- 活跃的社区支持
- 与 Rust 生态系统无缝集成

## 核心组件

Tokio 主要由以下几个部分组成：

### 运行时 (Runtime)

Tokio 运行时是整个异步系统的核心，负责：
- 执行异步任务
- 管理线程池
- 驱动 I/O 事件
- 调度任务执行

```rust
use tokio::runtime::Runtime;

// 创建一个新的运行时
let rt = Runtime::new().unwrap();

// 在运行时中执行异步代码
rt.block_on(async {
    println!("Hello from Tokio runtime!");
});
```

### 异步 I/O

Tokio 提供了异步版本的各种 I/O 操作：
- TCP/UDP 套接字
- 文件系统操作
- 进程管理
- 信号处理

### 任务 (Tasks)

任务是 Tokio 中并发执行的基本单位，类似于轻量级的绿色线程：

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let handle = task::spawn(async {
        // 这里的代码在后台异步执行
        println!("Hello from a task!");
        42
    });

    // 等待任务完成并获取结果
    let result = handle.await.unwrap();
    println!("Task result: {}", result);
}
```

### 调度器 (Scheduler)

工作窃取调度器确保任务在多个线程间高效分配：
- 每个线程有自己的任务队列
- 空闲线程可以从其他线程"窃取"任务
- 自动负载均衡

### 定时器 (Timers)

用于在将来的某个时间点执行代码：

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("开始等待...");
    sleep(Duration::from_secs(2)).await;
    println!("等待结束！");
}
```

## 何时**不**应该使用 Tokio

尽管 Tokio 功能强大，但并非适用于所有场景：

### CPU 密集型计算

对于纯计算任务，使用像 `rayon` 这样的并行计算库可能更合适：

```rust
// 不推荐：在 Tokio 中进行 CPU 密集型计算
#[tokio::main]
async fn main() {
    tokio::task::spawn(async {
        // 这会阻塞整个线程
        let result = expensive_computation();
        result
    }).await.unwrap();
}

// 推荐：使用 rayon 进行并行计算
use rayon::prelude::*;

fn main() {
    let results: Vec<i32> = (0..1000)
        .into_par_iter()
        .map(|i| expensive_computation(i))
        .collect();
}

fn expensive_computation(n: i32) -> i32 {
    // 复杂的计算逻辑
    n * n
}
```

### 大量文件读取

操作系统通常不提供异步文件 API，因此相对于标准线程池，Tokio 在这里没有明显优势。

### 单个网络请求

如果你只需要发送一个请求，使用 `reqwest` 的阻塞版本会更简单：

```rust
// 简单场景：使用阻塞版本
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://api.github.com/users/octocat")?;
    let user: serde_json::Value = response.json()?;
    println!("{:#}", user);
    Ok(())
}

// 复杂场景：需要并发请求时使用异步版本
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        "https://api.github.com/users/octocat",
        "https://api.github.com/users/defunkt",
        "https://api.github.com/users/pjhyett",
    ];

    let futures = urls.into_iter().map(|url| {
        tokio::spawn(async move {
            reqwest::get(url).await?.json::<serde_json::Value>().await
        })
    });

    let results = futures::future::try_join_all(futures).await?;
    for result in results {
        println!("{:#}", result?);
    }
    Ok(())
}
```

## async/await 入门

Rust 使用 `async/await` 关键字来实现异步编程。

### 基本概念

- `async fn`: 定义一个异步函数。调用它不会立即执行函数体，而是返回一个实现了 `Future` 特征的值。
- `.await`: 在一个 `Future` 上使用 `.await` 会暂停当前函数的执行，直到该 `Future` 完成。在此期间，Tokio 运行时可以执行其他任务。

### Future 特征

`Future` 是 Rust 异步编程的核心抽象：

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// 简化的 Future 定义
trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),    // Future 已完成
    Pending,     // Future 尚未完成
}
```

### 异步函数示例

```rust
use tokio::time::{sleep, Duration};

async fn say_world() {
    println!("world");
}

async fn say_hello_world() {
    println!("hello");
    say_world().await;
}

#[tokio::main]
async fn main() {
    say_hello_world().await;
}
```

### 错误处理

异步函数中的错误处理与同步代码类似：

```rust
use std::error::Error;
use tokio::fs;

async fn read_file_content(path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(path).await?;
    Ok(content)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match read_file_content("example.txt").await {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => eprintln!("读取文件失败: {}", e),
    }
    Ok(())
}
```

## 设置和配置

### 基本设置

要开始使用 Tokio，请将其添加到您的 `Cargo.toml` 文件中：

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

### 功能标志

`full` 功能标志会启用所有 Tokio 功能，方便入门。对于生产环境，您可能希望只启用所需的功能：

```toml
[dependencies]
tokio = { version = "1", features = [
    "macros",       # 启用 #[tokio::main] 宏
    "rt-multi-thread", # 多线程运行时
    "net",          # 网络功能
    "fs",           # 文件系统功能
    "time",         # 定时器功能
    "signal",       # 信号处理
] }
```

### 常用功能标志说明

- `macros`: 提供 `#[tokio::main]` 和 `#[tokio::test]` 宏
- `rt-multi-thread`: 多线程运行时（推荐用于服务器应用）
- `rt`: 单线程运行时（适用于简单应用）
- `net`: TCP/UDP 网络功能
- `fs`: 异步文件系统操作
- `time`: 定时器和延迟功能
- `signal`: Unix 信号处理
- `process`: 异步进程管理
- `sync`: 同步原语（如 Mutex、RwLock）

## Tokio 运行时详解

### 运行时类型

Tokio 提供了多种运行时配置：

#### 多线程运行时

```rust
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        println!("在多线程运行时中执行");
    });
}
```

#### 单线程运行时

```rust
use tokio::runtime;

fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    
    rt.block_on(async {
        println!("在单线程运行时中执行");
    });
}
```

#### 自定义运行时配置

```rust
use tokio::runtime;
use std::time::Duration;

fn main() {
    let rt = runtime::Builder::new_multi_thread()
        .worker_threads(4)                    // 设置工作线程数
        .thread_name("my-tokio-worker")       // 设置线程名称
        .thread_stack_size(3 * 1024 * 1024)  // 设置线程栈大小
        .enable_all()                         // 启用所有功能
        .build()
        .unwrap();
    
    rt.block_on(async {
        println!("在自定义运行时中执行");
    });
}
```

### 运行时句柄

可以获取运行时句柄来在其他线程中生成任务：

```rust
use tokio::runtime::Handle;
use std::thread;

#[tokio::main]
async fn main() {
    let handle = Handle::current();
    
    thread::spawn(move || {
        handle.spawn(async {
            println!("从其他线程生成的任务");
        });
    });
    
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}
```

## 异步 I/O 操作

### 文件系统操作

Tokio 提供了异步文件系统 API：

```rust
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 读取文件
    let content = fs::read_to_string("input.txt").await?;
    println!("文件内容: {}", content);
    
    // 写入文件
    fs::write("output.txt", "Hello, Tokio!").await?;
    
    // 使用 File 进行更复杂的操作
    let mut file = fs::File::create("example.txt").await?;
    file.write_all(b"Hello, world!").await?;
    file.flush().await?;
    
    Ok(())
}
```

### 缓冲 I/O

```rust
use tokio::fs::File;
use tokio::io::{BufReader, BufWriter, AsyncBufReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 缓冲读取
    let file = File::open("input.txt").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    while let Some(line) = lines.next_line().await? {
        println!("行: {}", line);
    }
    
    // 缓冲写入
    let file = File::create("output.txt").await?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(b"第一行\n").await?;
    writer.write_all(b"第二行\n").await?;
    writer.flush().await?;
    
    Ok(())
}
```

## 任务和并发

### 生成任务

使用 `tokio::spawn` 创建新任务：

```rust
use tokio::task;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let handle1 = task::spawn(async {
        for i in 1..=5 {
            println!("任务 1: {}", i);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        "任务 1 完成"
    });
    
    let handle2 = task::spawn(async {
        for i in 1..=3 {
            println!("任务 2: {}", i);
            tokio::time::sleep(Duration::from_millis(150)).await;
        }
        "任务 2 完成"
    });
    
    // 等待所有任务完成
    let (result1, result2) = tokio::join!(handle1, handle2);
    println!("{:?}", result1.unwrap());
    println!("{:?}", result2.unwrap());
}
```

### 任务取消

```rust
use tokio::task;
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let cloned_token = token.clone();
    
    let handle = task::spawn(async move {
        loop {
            tokio::select! {
                _ = sleep(Duration::from_millis(100)) => {
                    println!("任务正在运行...");
                }
                _ = cloned_token.cancelled() => {
                    println!("任务被取消");
                    break;
                }
            }
        }
    });
    
    // 2秒后取消任务
    sleep(Duration::from_secs(2)).await;
    token.cancel();
    
    handle.await.unwrap();
}
```

### 并发控制

#### 使用 Semaphore 限制并发数

```rust
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(3)); // 最多3个并发任务
    
    let mut handles = vec![];
    
    for i in 0..10 {
        let permit = semaphore.clone();
        let handle = tokio::spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            println!("任务 {} 开始执行", i);
            sleep(Duration::from_secs(1)).await;
            println!("任务 {} 完成", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
}
```

## 网络编程

### TCP 服务器

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("服务器监听在 127.0.0.1:8080");
    
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("新连接来自: {}", addr);
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("处理客户端时出错: {}", e);
            }
        });
    }
}

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; 1024];
    
    loop {
        let n = socket.read(&mut buf).await?;
        
        if n == 0 {
            break; // 连接关闭
        }
        
        // 回显数据
        socket.write_all(&buf[0..n]).await?;
    }
    
    Ok(())
}
```

### TCP 客户端

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("连接到服务器");
    
    // 发送数据
    stream.write_all(b"Hello, server!").await?;
    
    // 读取响应
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    
    println!("服务器响应: {}", String::from_utf8_lossy(&buf[0..n]));
    
    Ok(())
}
```

### UDP 套接字

```rust
use tokio::net::UdpSocket;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    println!("UDP 服务器监听在 127.0.0.1:8080");
    
    let mut buf = [0; 1024];
    
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("收到来自 {} 的数据: {}", addr, String::from_utf8_lossy(&buf[0..len]));
        
        // 回显数据
        socket.send_to(&buf[0..len], addr).await?;
    }
}
```

### HTTP 客户端示例

```rust
use reqwest;
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    
    // GET 请求
    let response = client
        .get("https://api.github.com/users/octocat")
        .header("User-Agent", "Tokio-Example")
        .send()
        .await?;
    
    let user: Value = response.json().await?;
    println!("用户信息: {:#}", user);
    
    // POST 请求
    let post_data = serde_json::json!({
        "title": "测试标题",
        "body": "测试内容",
        "userId": 1
    });
    
    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&post_data)
        .send()
        .await?;
    
    let created_post: Value = response.json().await?;
    println!("创建的帖子: {:#}", created_post);
    
    Ok(())
}
```

## 定时器和延迟

### 基本延迟

```rust
use tokio::time::{sleep, Duration, Instant};

#[tokio::main]
async fn main() {
    let start = Instant::now();
    
    println!("开始等待...");
    sleep(Duration::from_secs(2)).await;
    println!("等待结束，耗时: {:?}", start.elapsed());
}
```

### 定时器

```rust
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    let mut interval = interval(Duration::from_secs(1));
    
    for i in 0..5 {
        interval.tick().await;
        println!("定时器触发 #{}", i + 1);
    }
}
```

### 超时处理

```rust
use tokio::time::{timeout, Duration, sleep};

#[tokio::main]
async fn main() {
    let result = timeout(Duration::from_secs(2), long_running_task()).await;
    
    match result {
        Ok(value) => println!("任务完成，结果: {}", value),
        Err(_) => println!("任务超时"),
    }
}

async fn long_running_task() -> i32 {
    sleep(Duration::from_secs(3)).await; // 模拟长时间运行的任务
    42
}
```

### 延迟队列

```rust
use tokio::time::{sleep_until, Duration, Instant};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DelayedTask {
    execute_at: Instant,
    id: u32,
}

#[tokio::main]
async fn main() {
    let mut queue = BinaryHeap::new();
    let now = Instant::now();
    
    // 添加一些延迟任务
    queue.push(Reverse(DelayedTask {
        execute_at: now + Duration::from_secs(3),
        id: 1,
    }));
    queue.push(Reverse(DelayedTask {
        execute_at: now + Duration::from_secs(1),
        id: 2,
    }));
    queue.push(Reverse(DelayedTask {
        execute_at: now + Duration::from_secs(2),
        id: 3,
    }));
    
    while let Some(Reverse(task)) = queue.pop() {
        sleep_until(task.execute_at).await;
        println!("执行任务 {}", task.id);
    }
}
```

## 同步原语

### Mutex（互斥锁）

```rust
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = task::spawn(async move {
            let mut num = counter.lock().await;
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("最终计数: {}", *counter.lock().await);
}
```

### RwLock（读写锁）

```rust
use tokio::sync::RwLock;
use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // 多个读取任务
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = task::spawn(async move {
            let read_guard = data.read().await;
            println!("读取任务 {}: {:?}", i, *read_guard);
        });
        handles.push(handle);
    }
    
    // 一个写入任务
    let data_clone = Arc::clone(&data);
    let write_handle = task::spawn(async move {
        let mut write_guard = data_clone.write().await;
        write_guard.push(6);
        println!("写入任务完成");
    });
    handles.push(write_handle);
    
    for handle in handles {
        handle.await.unwrap();
    }
}
```

### Channel（通道）

#### MPSC（多生产者单消费者）

```rust
use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    
    // 生产者任务
    for i in 0..5 {
        let tx = tx.clone();
        task::spawn(async move {
            tx.send(format!("消息 {}", i)).await.unwrap();
        });
    }
    
    // 关闭发送端
    drop(tx);
    
    // 消费者
    while let Some(message) = rx.recv().await {
        println!("收到: {}", message);
    }
}
```

#### Oneshot（一次性通道）

```rust
use tokio::sync::oneshot;
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();
    
    task::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        tx.send("计算结果").unwrap();
    });
    
    match rx.await {
        Ok(result) => println!("收到结果: {}", result),
        Err(_) => println!("发送端已关闭"),
    }
}
```

#### Broadcast（广播通道）

```rust
use tokio::sync::broadcast;
use tokio::task;

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel(16);
    
    // 创建多个接收者
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();
    let mut rx3 = tx.subscribe();
    
    // 接收者任务
    let handle1 = task::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("接收者1收到: {}", msg);
        }
    });
    
    let handle2 = task::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("接收者2收到: {}", msg);
        }
    });
    
    let handle3 = task::spawn(async move {
        while let Ok(msg) = rx3.recv().await {
            println!("接收者3收到: {}", msg);
        }
    });
    
    // 发送消息
    for i in 0..5 {
        tx.send(format!("广播消息 {}", i)).unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    drop(tx); // 关闭发送端
    
    let _ = tokio::join!(handle1, handle2, handle3);
}
```

### Watch（监视通道）

```rust
use tokio::sync::watch;
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel("初始值");
    
    // 监听者任务
    let handle = task::spawn(async move {
        while rx.changed().await.is_ok() {
            println!("值已更改为: {}", *rx.borrow());
        }
    });
    
    // 更新值
    for i in 1..=5 {
        sleep(Duration::from_secs(1)).await;
        tx.send(format!("值 {}", i)).unwrap();
    }
    
    drop(tx); // 关闭发送端
    handle.await.unwrap();
}
```

## 错误处理

### 基本错误处理

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "自定义错误: {}", self.message)
    }
}

impl Error for CustomError {}

async fn might_fail(should_fail: bool) -> Result<String, CustomError> {
    if should_fail {
        Err(CustomError {
            message: "操作失败".to_string(),
        })
    } else {
        Ok("操作成功".to_string())
    }
}

#[tokio::main]
async fn main() {
    match might_fail(false).await {
        Ok(result) => println!("成功: {}", result),
        Err(e) => eprintln!("错误: {}", e),
    }
    
    match might_fail(true).await {
        Ok(result) => println!("成功: {}", result),
        Err(e) => eprintln!("错误: {}", e),
    }
}
```

### 使用 anyhow 简化错误处理

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

```rust
use anyhow::{Result, Context};
use tokio::fs;

async fn read_config() -> Result<String> {
    let content = fs::read_to_string("config.toml")
        .await
        .context("无法读取配置文件")?;
    
    if content.is_empty() {
        anyhow::bail!("配置文件为空");
    }
    
    Ok(content)
}

#[tokio::main]
async fn main() -> Result<()> {
    match read_config().await {
        Ok(config) => println!("配置: {}", config),
        Err(e) => {
            eprintln!("错误: {}", e);
            for cause in e.chain().skip(1) {
                eprintln!("原因: {}", cause);
            }
        }
    }
    
    Ok(())
}
```

### 错误传播和转换

```rust
use std::error::Error;
use std::fmt;
use tokio::fs;
use serde_json;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "I/O 错误: {}", e),
            AppError::Json(e) => write!(f, "JSON 错误: {}", e),
            AppError::Custom(msg) => write!(f, "应用错误: {}", msg),
        }
    }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::Json(error)
    }
}

async fn process_json_file(path: &str) -> Result<serde_json::Value, AppError> {
    let content = fs::read_to_string(path).await?;
    let json: serde_json::Value = serde_json::from_str(&content)?;
    Ok(json)
}

#[tokio::main]
async fn main() {
    match process_json_file("data.json").await {
        Ok(json) => println!("JSON 数据: {:#}", json),
        Err(e) => eprintln!("处理失败: {}", e),
    }
}
```

## 性能优化

### 避免不必要的分配

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::error::Error;

// 不好的做法：每次都分配新的缓冲区
async fn bad_echo_server(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        let mut buf = vec![0; 1024]; // 每次循环都分配
        let n = stream.read(&mut buf).await?;
        if n == 0 { break; }
        stream.write_all(&buf[0..n]).await?;
    }
    Ok(())
}

// 好的做法：重用缓冲区
async fn good_echo_server(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; 1024]; // 只分配一次
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 { break; }
        stream.write_all(&buf[0..n]).await?;
    }
    Ok(())
}
```

### 使用对象池

```rust
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::VecDeque;

struct BufferPool {
    buffers: Mutex<VecDeque<Vec<u8>>>,
}

impl BufferPool {
    fn new() -> Self {
        Self {
            buffers: Mutex::new(VecDeque::new()),
        }
    }
    
    async fn get_buffer(&self) -> Vec<u8> {
        let mut buffers = self.buffers.lock().await;
        buffers.pop_front().unwrap_or_else(|| vec![0; 1024])
    }
    
    async fn return_buffer(&self, mut buffer: Vec<u8>) {
        buffer.clear();
        buffer.resize(1024, 0);
        let mut buffers = self.buffers.lock().await;
        if buffers.len() < 10 { // 限制池大小
            buffers.push_back(buffer);
        }
    }
}

#[tokio::main]
async fn main() {
    let pool = Arc::new(BufferPool::new());
    
    // 使用缓冲池
    let buffer = pool.get_buffer().await;
    // ... 使用缓冲区 ...
    pool.return_buffer(buffer).await;
}
```

### 批量处理

```rust
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use std::collections::VecDeque;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1000);
    
    // 生产者
    tokio::spawn(async move {
        for i in 0..1000 {
            tx.send(i).await.unwrap();
            if i % 100 == 0 {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }
    });
    
    // 批量消费者
    let mut batch = VecDeque::new();
    let mut interval = interval(Duration::from_millis(100));
    
    loop {
        tokio::select! {
            // 收集消息
            msg = rx.recv() => {
                match msg {
                    Some(value) => {
                        batch.push_back(value);
                        if batch.len() >= 50 { // 批量大小
                            process_batch(&mut batch).await;
                        }
                    }
                    None => {
                        if !batch.is_empty() {
                            process_batch(&mut batch).await;
                        }
                        break;
                    }
                }
            }
            // 定时处理
            _ = interval.tick() => {
                if !batch.is_empty() {
                    process_batch(&mut batch).await;
                }
            }
        }
    }
}

async fn process_batch(batch: &mut VecDeque<i32>) {
    println!("处理批量数据，大小: {}", batch.len());
    // 模拟批量处理
    tokio::time::sleep(Duration::from_millis(10)).await;
    batch.clear();
}
```

## 最佳实践

### 1. 合理选择运行时配置

```rust
// 对于 I/O 密集型应用
let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(num_cpus::get())
    .enable_all()
    .build()
    .unwrap();

// 对于简单的单线程应用
let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();
```

### 2. 避免阻塞操作

```rust
// 错误：在异步上下文中使用阻塞操作
#[tokio::main]
async fn main() {
    // 这会阻塞整个线程
    std::thread::sleep(std::time::Duration::from_secs(1));
}

// 正确：使用异步版本
#[tokio::main]
async fn main() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

// 如果必须使用阻塞操作，使用 spawn_blocking
#[tokio::main]
async fn main() {
    let result = tokio::task::spawn_blocking(|| {
        // 阻塞操作
        std::thread::sleep(std::time::Duration::from_secs(1));
        "完成"
    }).await.unwrap();
    
    println!("结果: {}", result);
}
```

### 3. 正确处理错误

```rust
use anyhow::Result;

// 使用 ? 操作符传播错误
async fn fetch_data() -> Result<String> {
    let response = reqwest::get("https://api.example.com/data").await?;
    let text = response.text().await?;
    Ok(text)
}

// 在适当的地方处理错误
#[tokio::main]
async fn main() {
    match fetch_data().await {
        Ok(data) => println!("数据: {}", data),
        Err(e) => eprintln!("获取数据失败: {}", e),
    }
}
```

### 4. 使用结构化并发

```rust
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut set = JoinSet::new();
    
    // 添加任务到集合
    for i in 0..5 {
        set.spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(i * 100)).await;
            format!("任务 {} 完成", i)
        });
    }
    
    // 等待所有任务完成
    while let Some(res) = set.join_next().await {
        match res {
            Ok(output) => println!("{}", output),
            Err(e) => eprintln!("任务失败: {}", e),
        }
    }
    
    Ok(())
}
```

### 5. 资源管理

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

struct ResourceManager {
    semaphore: Arc<Semaphore>,
}

impl ResourceManager {
    fn new(max_resources: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_resources)),
        }
    }
    
    async fn acquire_resource(&self) -> ResourceGuard {
        let permit = self.semaphore.acquire().await.unwrap();
        ResourceGuard { _permit: permit }
    }
}

struct ResourceGuard {
    _permit: tokio::sync::SemaphorePermit<'static>,
}

#[tokio::main]
async fn main() {
    let manager = ResourceManager::new(3);
    
    let _guard = manager.acquire_resource().await;
    // 使用资源
    // guard 被丢弃时自动释放资源
}
```

## 常见问题和解决方案

### 问题1：任务泄漏

```rust
// 问题：忘记等待任务完成
#[tokio::main]
async fn main() {
    tokio::spawn(async {
        println!("这个任务可能不会完成");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    });
    // 主函数立即结束，任务可能被取消
}

// 解决方案：等待任务完成
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("这个任务会完成");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    });
    
    handle.await.unwrap();
}
```

### 问题2：死锁

```rust
use tokio::sync::Mutex;
use std::sync::Arc;

// 问题：可能导致死锁的代码
async fn deadlock_example() {
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));
    
    let mutex1_clone = mutex1.clone();
    let mutex2_clone = mutex2.clone();
    
    let task1 = tokio::spawn(async move {
        let _guard1 = mutex1_clone.lock().await;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let _guard2 = mutex2_clone.lock().await;
    });
    
    let task2 = tokio::spawn(async move {
        let _guard2 = mutex2.lock().await;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let _guard1 = mutex1.lock().await;
    });
    
    let _ = tokio::join!(task1, task2);
}

// 解决方案：始终以相同顺序获取锁
async fn no_deadlock_example() {
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));
    
    let mutex1_clone = mutex1.clone();
    let mutex2_clone = mutex2.clone();
    
    let task1 = tokio::spawn(async move {
        let _guard1 = mutex1_clone.lock().await;
        let _guard2 = mutex2_clone.lock().await;
        // 使用两个锁
    });
    
    let task2 = tokio::spawn(async move {
        let _guard1 = mutex1.lock().await;  // 相同顺序
        let _guard2 = mutex2.lock().await;
        // 使用两个锁
    });
    
    let _ = tokio::join!(task1, task2);
}
```

### 问题3：内存泄漏

```rust
use tokio::sync::mpsc;
use std::sync::Arc;

// 问题：循环引用导致内存泄漏
struct Node {
    data: i32,
    next: Option<Arc<Node>>,
    sender: mpsc::Sender<i32>,
}

// 解决方案：使用 Weak 引用打破循环
use std::sync::Weak;

struct BetterNode {
    data: i32,
    next: Option<Weak<BetterNode>>, // 使用 Weak 引用
    sender: mpsc::Sender<i32>,
}
```

## 生态系统

### 核心库

- **tokio**: 异步运行时
- **tokio-util**: Tokio 实用工具
- **tokio-stream**: 异步流处理
- **tokio-test**: 测试工具

### 网络库

- **hyper**: HTTP 客户端和服务器
- **reqwest**: 高级 HTTP 客户端
- **tonic**: gRPC 实现
- **warp**: Web 框架
- **axum**: 现代 Web 框架

### 数据库

- **sqlx**: 异步 SQL 工具包
- **sea-orm**: 异步 ORM
- **redis**: Redis 客户端
- **mongodb**: MongoDB 驱动

### 序列化

- **serde**: 序列化框架
- **serde_json**: JSON 支持
- **bincode**: 二进制序列化
- **postcard**: 嵌入式友好的序列化

## 实际项目示例

### 简单的 Web 服务器

```rust
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match req.uri().path() {
        "/" => Ok(Response::new(Body::from("Hello, World!")));
        "/health" => Ok(Response::new(Body::from("OK")));
        _ => {
            let mut response = Response::new(Body::from("Not Found"));
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });
    
    let server = Server::bind(&addr).serve(make_svc);
    
    println!("服务器运行在 http://{}", addr);
    
    if let Err(e) = server.await {
        eprintln!("服务器错误: {}", e);
    }
    
    Ok(())
}
```

### 聊天服务器

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::collections::HashMap;
use std::sync::Arc;

type Clients = Arc<Mutex<HashMap<String, TcpStream>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, _rx) = broadcast::channel(100);
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    
    println!("聊天服务器启动在 127.0.0.1:8080");
    
    while let Ok((stream, addr)) = listener.accept().await {
        let tx = tx.clone();
        let clients = clients.clone();
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, addr.to_string(), tx, clients).await {
                eprintln!("客户端错误: {}", e);
            }
        });
    }
    
    Ok(())
}

async fn handle_client(
    mut stream: TcpStream,
    addr: String,
    tx: broadcast::Sender<String>,
    clients: Clients,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rx = tx.subscribe();
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    
    // 添加客户端
    {
        let mut clients_guard = clients.lock().await;
        clients_guard.insert(addr.clone(), stream.try_clone()?);
    }
    
    loop {
        tokio::select! {
            // 读取客户端消息
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => break, // 连接关闭
                    Ok(_) => {
                        let message = format!("{}: {}", addr, line.trim());
                        tx.send(message)?;
                        line.clear();
                    }
                    Err(e) => {
                        eprintln!("读取错误: {}", e);
                        break;
                    }
                }
            }
            // 广播消息给客户端
            result = rx.recv() => {
                match result {
                    Ok(message) => {
                        if let Err(e) = writer.write_all(message.as_bytes()).await {
                            eprintln!("写入错误: {}", e);
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
    
    // 移除客户端
    {
        let mut clients_guard = clients.lock().await;
        clients_guard.remove(&addr);
    }
    
    Ok(())
}
```

### 文件下载器

```rust
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use indicatif::{ProgressBar, ProgressStyle};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://releases.ubuntu.com/20.04/ubuntu-20.04.6-desktop-amd64.iso";
    let filename = "ubuntu.iso";
    
    download_file(url, filename).await?;
    
    Ok(())
}

async fn download_file(url: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    
    let total_size = response.content_length().unwrap_or(0);
    
    // 创建进度条
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    
    let mut file = File::create(filename).await?;
    let mut downloaded = 0u64;
    let mut stream = response.bytes_stream();
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }
    
    pb.finish_with_message("下载完成");
    println!("文件已保存为: {}", filename);
    
    Ok(())
}
```

### 负载均衡器

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::error::Error;

struct LoadBalancer {
    backends: Vec<String>,
    current: AtomicUsize,
}

impl LoadBalancer {
    fn new(backends: Vec<String>) -> Self {
        Self {
            backends,
            current: AtomicUsize::new(0),
        }
    }
    
    fn next_backend(&self) -> &str {
        let index = self.current.fetch_add(1, Ordering::Relaxed) % self.backends.len();
        &self.backends[index]
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let backends = vec![
        "127.0.0.1:8081".to_string(),
        "127.0.0.1:8082".to_string(),
        "127.0.0.1:8083".to_string(),
    ];
    
    let load_balancer = Arc::new(LoadBalancer::new(backends));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    println!("负载均衡器启动在 127.0.0.1:8080");
    
    while let Ok((client_stream, _)) = listener.accept().await {
        let lb = load_balancer.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(client_stream, lb).await {
                eprintln!("连接处理错误: {}", e);
            }
        });
    }
    
    Ok(())
}

async fn handle_connection(
    mut client_stream: TcpStream,
    load_balancer: Arc<LoadBalancer>,
) -> Result<(), Box<dyn Error>> {
    let backend_addr = load_balancer.next_backend();
    let mut backend_stream = TcpStream::connect(backend_addr).await?;
    
    let (mut client_read, mut client_write) = client_stream.split();
    let (mut backend_read, mut backend_write) = backend_stream.split();
    
    // 双向代理
    let client_to_backend = tokio::spawn(async move {
        let mut buffer = [0; 4096];
        loop {
            match client_read.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    if backend_write.write_all(&buffer[0..n]).await.is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });
    
    let backend_to_client = tokio::spawn(async move {
        let mut buffer = [0; 4096];
        loop {
            match backend_read.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    if client_write.write_all(&buffer[0..n]).await.is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });
    
    let _ = tokio::join!(client_to_backend, backend_to_client);
    Ok(())
}
```

## 测试

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};
    
    #[tokio::test]
    async fn test_async_function() {
        let result = async_add(2, 3).await;
        assert_eq!(result, 5);
    }
    
    #[tokio::test]
    async fn test_timeout() {
        let result = tokio::time::timeout(
            Duration::from_millis(100),
            sleep(Duration::from_millis(200))
        ).await;
        
        assert!(result.is_err()); // 应该超时
    }
    
    async fn async_add(a: i32, b: i32) -> i32 {
        sleep(Duration::from_millis(10)).await;
        a + b
    }
}
```

### 集成测试

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::test]
async fn test_echo_server() -> Result<(), Box<dyn Error>> {
    // 启动测试服务器
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    
    tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                loop {
                    match socket.read(&mut buf).await {
                        Ok(0) => break,
                        Ok(n) => {
                            if socket.write_all(&buf[0..n]).await.is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
        }
    });
    
    // 测试客户端
    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(b"Hello, World!").await?;
    
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    
    assert_eq!(&buf[0..n], b"Hello, World!");
    
    Ok(())
}
```

## 调试和监控

### 日志记录

```rust
use tracing::{info, warn, error, debug};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    info!("应用程序启动");
    
    match risky_operation().await {
        Ok(result) => info!("操作成功: {}", result),
        Err(e) => error!("操作失败: {}", e),
    }
}

async fn risky_operation() -> Result<String, &'static str> {
    debug!("开始执行危险操作");
    
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    if rand::random::<bool>() {
        warn!("操作可能失败");
        Err("随机失败")
    } else {
        Ok("成功".to_string())
    }
}
```

### 性能监控

```rust
use tokio::time::{Instant, Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

struct Metrics {
    requests_total: AtomicU64,
    requests_duration_sum: AtomicU64,
}

impl Metrics {
    fn new() -> Self {
        Self {
            requests_total: AtomicU64::new(0),
            requests_duration_sum: AtomicU64::new(0),
        }
    }
    
    fn record_request(&self, duration: Duration) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        self.requests_duration_sum.fetch_add(
            duration.as_millis() as u64,
            Ordering::Relaxed
        );
    }
    
    fn average_duration(&self) -> f64 {
        let total = self.requests_total.load(Ordering::Relaxed);
        let sum = self.requests_duration_sum.load(Ordering::Relaxed);
        
        if total == 0 {
            0.0
        } else {
            sum as f64 / total as f64
        }
    }
}

#[tokio::main]
async fn main() {
    let metrics = Arc::new(Metrics::new());
    
    // 模拟一些请求
    for _ in 0..100 {
        let metrics = metrics.clone();
        tokio::spawn(async move {
            let start = Instant::now();
            
            // 模拟请求处理
            tokio::time::sleep(Duration::from_millis(10)).await;
            
            metrics.record_request(start.elapsed());
        });
    }
    
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    println!("总请求数: {}", metrics.requests_total.load(Ordering::Relaxed));
    println!("平均响应时间: {:.2}ms", metrics.average_duration());
}
```

## 部署和生产环境

### Docker 化

```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/my-tokio-app /usr/local/bin/my-tokio-app

EXPOSE 8080

CMD ["my-tokio-app"]
```

### 配置管理

```rust
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
    logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct LoggingConfig {
    level: String,
    format: String,
}

impl Config {
    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "config.toml".to_string());
        
        let config_str = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_str)?;
        
        Ok(config)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    
    println!("服务器配置: {:?}", config.server);
    println!("数据库配置: {:?}", config.database);
    
    // 使用配置启动应用
    start_server(config).await?;
    
    Ok(())
}

async fn start_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", config.server.host, config.server.port);
    println!("服务器启动在: {}", addr);
    
    // 实际的服务器启动逻辑
    Ok(())
}
```

## 总结

Tokio 是 Rust 生态系统中最重要的异步运行时，它提供了：

1. **高性能的异步运行时**：基于工作窃取调度器，能够高效处理大量并发任务
2. **丰富的异步 I/O 支持**：包括网络、文件系统、进程等
3. **强大的同步原语**：Mutex、RwLock、Channel 等
4. **优秀的错误处理机制**：与 Rust 的错误处理模型完美集成
5. **活跃的生态系统**：大量高质量的第三方库

通过本指南，你应该能够：
- 理解 Tokio 的核心概念和架构
- 掌握异步编程的最佳实践
- 避免常见的陷阱和问题
- 构建高性能的异步应用程序

记住，异步编程需要不同的思维方式，但一旦掌握，它将为你的应用程序带来卓越的性能和可扩展性。继续实践和探索 Tokio 的强大功能，你将能够构建出色的 Rust 应用程序。

## 参考资源

- [Tokio 官方文档](https://tokio.rs/)
- [Tokio GitHub 仓库](https://github.com/tokio-rs/tokio)
- [Rust 异步编程指南](https://rust-lang.github.io/async-book/)
- [Tokio 教程](https://tokio.rs/tokio/tutorial)
- [Rust 官方文档](https://doc.rust-lang.org/)

---

*本指南涵盖了 Tokio 的主要功能和用法，但 Tokio 是一个不断发展的项目。建议定期查看官方文档以获取最新信息和功能更新。*