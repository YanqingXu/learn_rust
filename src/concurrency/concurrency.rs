//! # Rust 并发编程学习
//! 
//! 本模块演示 Rust 中的并发编程特性，包括：
//! - 线程 (Threads)
//! - 消息传递 (Message Passing)
//! - 共享状态 (Shared State)
//! - 同步原语 (Synchronization Primitives)
//! 
//! Rust 的所有权系统确保了并发安全

use std::sync::{Arc, Mutex, mpsc, RwLock, Barrier};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    println!("=== Rust 并发编程学习 ===\n");
    
    // 1. 基本线程操作
    println!("1. 基本线程操作：");
    basic_threads_demo();
    println!();
    
    // 2. 消息传递
    println!("2. 消息传递：");
    message_passing_demo();
    println!();
    
    // 3. 共享状态
    println!("3. 共享状态：");
    shared_state_demo();
    println!();
    
    // 4. 高级并发模式
    println!("4. 高级并发模式：");
    advanced_concurrency_demo();
}

/// 基本线程操作演示
fn basic_threads_demo() {
    // 创建简单线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("  子线程: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });
    
    for i in 1..5 {
        println!("  主线程: {}", i);
        thread::sleep(Duration::from_millis(50));
    }
    
    // 等待线程完成
    handle.join().unwrap();
    println!("  子线程执行完毕");
    
    // 线程间移动数据
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("  线程中的向量: {:?}", v);
    });
    
    handle.join().unwrap();
    
    // 多个线程
    let mut handles = vec![];
    
    for i in 0..3 {
        let handle = thread::spawn(move || {
            println!("  线程 {} 开始执行", i);
            thread::sleep(Duration::from_millis(100));
            println!("  线程 {} 执行完毕", i);
            i * 2
        });
        handles.push(handle);
    }
    
    // 收集结果
    println!("  等待所有线程完成...");
    for handle in handles {
        let result = handle.join().unwrap();
        println!("  线程返回结果: {}", result);
    }
}

/// 消息传递演示
fn message_passing_demo() {
    // 基本的消息传递
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hello from thread");
        tx.send(val).unwrap();
        println!("  发送方: 消息已发送");
    });
    
    let received = rx.recv().unwrap();
    println!("  接收方: 收到消息 '{}'", received);
    
    // 发送多个消息
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        println!("  发送方: 所有消息已发送");
    });
    
    println!("  接收方: 开始接收消息");
    for received in rx {
        println!("  接收到: {}", received);
    }
    
    // 多个发送方
    let (tx, rx) = mpsc::channel();
    
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("线程1: hello"),
            String::from("线程1: world"),
        ];
        
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("线程2: rust"),
            String::from("线程2: programming"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });
    
    // 接收来自多个发送方的消息
    println!("  从多个发送方接收消息:");
    for received in rx {
        println!("  收到: {}", received);
    }
}

/// 共享状态演示
fn shared_state_demo() {
    // 基本的 Mutex 使用
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("  线程更新计数器: {}", *num);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("  最终计数器值: {}", *counter.lock().unwrap());
    
    // 共享复杂数据结构
    let data = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];
    
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut map = data.lock().unwrap();
            map.insert(format!("key_{}", i), i * 10);
            println!("  线程 {} 插入数据", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_data = data.lock().unwrap();
    println!("  最终数据: {:?}", *final_data);
    
    // RwLock 读写锁演示
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // 多个读者
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let reader = data.read().unwrap();
            println!("  读者 {} 读取数据: {:?}", i, *reader);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // 一个写者
    let data_writer = Arc::clone(&data);
    let write_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut writer = data_writer.write().unwrap();
        writer.push(6);
        println!("  写者添加元素 6");
    });
    handles.push(write_handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_data = data.read().unwrap();
    println!("  RwLock 最终数据: {:?}", *final_data);
}

/// 高级并发模式演示
fn advanced_concurrency_demo() {
    // 工作者池模式
    println!("  工作者池模式:");
    worker_pool_demo();
    
    // 屏障同步
    println!("  屏障同步:");
    barrier_demo();
    
    // 生产者-消费者模式
    println!("  生产者-消费者模式:");
    producer_consumer_demo();
    
    // 原子操作
    println!("  原子操作:");
    atomic_demo();
}

/// 工作者池演示
fn worker_pool_demo() {
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = vec![];
    
    // 创建工作者线程
    for id in 0..3 {
        let receiver = Arc::clone(&receiver);
        let worker = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("  工作者 {} 收到任务: {}", id, job);
                        // 模拟工作
                        thread::sleep(Duration::from_millis(200));
                        println!("  工作者 {} 完成任务: {}", id, job);
                    }
                    Err(_) => {
                        println!("  工作者 {} 断开连接", id);
                        break;
                    }
                }
            }
        });
        workers.push(worker);
    }
    
    // 发送任务
    for i in 1..=6 {
        sender.send(format!("任务{}", i)).unwrap();
    }
    
    // 关闭发送端
    drop(sender);
    
    // 等待所有工作者完成
    for worker in workers {
        worker.join().unwrap();
    }
}

/// 屏障同步演示
fn barrier_demo() {
    let n = 3;
    let barrier = Arc::new(Barrier::new(n));
    let mut handles = vec![];
    
    for i in 0..n {
        let c = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("  线程 {} 开始工作", i);
            thread::sleep(Duration::from_millis((i as u64 + 1) * 100));
            println!("  线程 {} 工作完成，等待其他线程", i);
            
            c.wait();
            
            println!("  线程 {} 所有线程都完成了，继续执行", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// 生产者-消费者模式演示
fn producer_consumer_demo() {
    let (tx, rx) = mpsc::channel();
    let buffer_size = 5;
    
    // 生产者
    let producer = thread::spawn(move || {
        for i in 0..10 {
            println!("  生产者: 生产物品 {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
        println!("  生产者: 生产完毕");
    });
    
    // 消费者
    let consumer = thread::spawn(move || {
        while let Ok(item) = rx.recv() {
            println!("  消费者: 消费物品 {}", item);
            thread::sleep(Duration::from_millis(200));
        }
        println!("  消费者: 消费完毕");
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}

/// 原子操作演示
fn atomic_demo() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("  原子计数器最终值: {}", counter.load(Ordering::SeqCst));
    
    // 比较并交换
    let value = Arc::new(AtomicUsize::new(10));
    let value_clone = Arc::clone(&value);
    
    let handle = thread::spawn(move || {
        let old_value = value_clone.compare_exchange(10, 20, Ordering::SeqCst, Ordering::SeqCst);
        match old_value {
            Ok(v) => println!("  比较并交换成功，旧值: {}", v),
            Err(v) => println!("  比较并交换失败，当前值: {}", v),
        }
    });
    
    handle.join().unwrap();
    println!("  比较并交换后的值: {}", value.load(Ordering::SeqCst));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    #[test]
    fn test_basic_threading() {
        let handle = thread::spawn(|| {
            42
        });
        
        let result = handle.join().unwrap();
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_message_passing() {
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            tx.send("hello").unwrap();
        });
        
        let received = rx.recv().unwrap();
        assert_eq!(received, "hello");
    }
    
    #[test]
    fn test_shared_state() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(*counter.lock().unwrap(), 10);
    }
    
    #[test]
    fn test_rwlock() {
        let data = Arc::new(RwLock::new(vec![1, 2, 3]));
        let data_clone = Arc::clone(&data);
        
        let handle = thread::spawn(move || {
            let reader = data_clone.read().unwrap();
            assert_eq!(*reader, vec![1, 2, 3]);
        });
        
        handle.join().unwrap();
        
        {
            let mut writer = data.write().unwrap();
            writer.push(4);
        }
        
        let reader = data.read().unwrap();
        assert_eq!(*reader, vec![1, 2, 3, 4]);
    }
    
    #[test]
    fn test_atomic_operations() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.load(Ordering::SeqCst), 1000);
    }
    
    #[test]
    fn test_compare_exchange() {
        let value = AtomicUsize::new(10);
        let old = value.compare_exchange(10, 20, Ordering::SeqCst, Ordering::SeqCst);
        assert_eq!(old, Ok(10));
        assert_eq!(value.load(Ordering::SeqCst), 20);
    }
    
    #[test]
    fn test_barrier() {
        let n = 3;
        let barrier = Arc::new(Barrier::new(n));
        let mut handles = vec![];
        
        for i in 0..n {
            let c = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                c.wait();
                i
            });
            handles.push(handle);
        }
        
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(results.len(), 3);
    }
    
    #[test]
    fn test_multiple_producers() {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        
        thread::spawn(move || {
            tx1.send(1).unwrap();
        });
        
        thread::spawn(move || {
            tx.send(2).unwrap();
        });
        
        let mut results = vec![];
        results.push(rx.recv().unwrap());
        results.push(rx.recv().unwrap());
        
        results.sort();
        assert_eq!(results, vec![1, 2]);
    }
}
