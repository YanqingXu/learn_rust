//! # Rust 函数式编程学习
//!
//! 本模块演示 Rust 中的函数式编程特性，包括：
//! - 闭包 (Closures)
//! - 迭代器 (Iterators)
//! - 函数式编程模式
//!
//! Rust 支持函数式和命令式编程风格的混合

use std::collections::HashMap;
use std::thread;

fn main() {
    println!("=== Rust 函数式编程学习 ===\n");

    // 1. 闭包演示
    println!("1. 闭包 (Closures)：");
    closure_demo();
    println!();

    // 2. 迭代器演示
    println!("2. 迭代器 (Iterators)：");
    iterator_demo();
    println!();

    // 3. 函数式编程模式
    println!("3. 函数式编程模式：");
    functional_patterns_demo();
    println!();

    // 4. 高级函数式技巧
    println!("4. 高级函数式技巧：");
    advanced_functional_demo();
}

/// 闭包演示
fn closure_demo() {
    // 基本闭包
    let expensive_closure = |num| {
        println!("  计算中...");
        thread::sleep(std::time::Duration::from_millis(100));
        num
    };

    println!("  调用闭包: {}", expensive_closure(5));

    // 闭包捕获环境
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
    println!("  闭包捕获环境: {} == {} 是 {}", y, x, equal_to_x(y));

    // 不同的闭包类型推断
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    println!("  类型推断闭包: '{}'", s);

    // 缓存闭包结果
    let mut expensive_result = Cacher::new(|a| {
        println!("  执行昂贵计算...");
        thread::sleep(std::time::Duration::from_millis(50));
        a * 2
    });

    println!(
        "  第一次调用 expensive_result.value(10): {}",
        expensive_result.value(10)
    );
    println!(
        "  第二次调用 expensive_result.value(10): {}",
        expensive_result.value(10)
    );

    // move 闭包
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x); // 这行会编译错误
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    println!("  move 闭包成功执行");

    // 作为函数参数的闭包
    let list = vec![1, 2, 3];
    println!("  排序前: {:?}", list);

    let mut list = list;
    list.sort_by_key(|item| *item);
    println!("  按绝对值排序后: {:?}", list);
}

/// 缓存结构体
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

/// 迭代器演示
fn iterator_demo() {
    // 基本迭代器
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    println!("  遍历 Vector:");
    for val in v1_iter {
        println!("    值: {}", val);
    }

    // 迭代器适配器
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("  map 适配器: {:?} -> {:?}", v1, v2);

    // 过滤器
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let v2: Vec<_> = v1.into_iter().filter(|x| *x > 3).collect();
    println!("  filter 适配器: 过滤 > 3 的元素: {:?}", v2);

    // 链式调用
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result: Vec<_> = v1.iter().map(|x| x * x).filter(|&x| x > 5).collect();
    println!("  链式调用 (平方后过滤 > 5): {:?}", result);

    // fold 和 reduce
    let v = vec![1, 2, 3, 4, 5];
    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("  fold 求和: {}", sum);

    let product = v.iter().fold(1, |acc, x| acc * x);
    println!("  fold 求积: {}", product);

    // find 和 any
    let v = vec![1, 2, 3, 4, 5];
    let found = v.iter().find(|&&x| x > 3);
    println!("  find 第一个 > 3 的元素: {:?}", found);

    let any_even = v.iter().any(|&x| x % 2 == 0);
    println!("  any 是否有偶数: {}", any_even);

    let all_positive = v.iter().all(|&x| x > 0);
    println!("  all 是否都为正数: {}", all_positive);

    // enumerate
    let v = vec!["apple", "banana", "cherry"];
    println!("  enumerate 枚举:");
    for (index, value) in v.iter().enumerate() {
        println!("    索引 {}: {}", index, value);
    }

    // zip
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];
    let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("  zip 配对: {:?}", pairs);
}

/// 函数式编程模式演示
fn functional_patterns_demo() {
    // 函数作为参数
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled = apply_operation(&numbers, |x| x * 2);
    println!("  函数作为参数 - 翻倍: {:?}", doubled);

    let squared = apply_operation(&numbers, |x| x * x);
    println!("  函数作为参数 - 平方: {:?}", squared);

    // 函数作为返回值
    let add_one = create_adder(1);
    let add_five = create_adder(5);

    println!("  返回函数 - add_one(10): {}", add_one(10));
    println!("  返回函数 - add_five(10): {}", add_five(10));

    // 组合函数
    let add_two = |x| x + 2;
    let multiply_three = |x| x * 3;

    let composed = compose(add_two, multiply_three);
    println!("  函数组合 - (5 + 2) * 3: {}", composed(5));

    // 数据处理管道
    let words = vec!["hello", "world", "rust", "programming"];
    let result = words
        .iter()
        .filter(|word| word.len() > 4)
        .map(|word| word.to_uppercase())
        .collect::<Vec<String>>()
        .join(" ");
    println!("  数据处理管道: '{}'", result);

    // 分组操作
    let people = vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
        },
        Person {
            name: "Charlie".to_string(),
            age: 30,
        },
        Person {
            name: "David".to_string(),
            age: 35,
        },
    ];

    let grouped = group_by(people, |person| person.age);
    println!("  按年龄分组:");
    for (age, people) in grouped {
        println!(
            "    年龄 {}: {:?}",
            age,
            people.iter().map(|p| &p.name).collect::<Vec<_>>()
        );
    }
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

/// 应用操作到向量的每个元素
fn apply_operation<F>(numbers: &[i32], operation: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    numbers.iter().map(|&x| operation(x)).collect()
}

/// 创建加法器函数
fn create_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

/// 函数组合
fn compose<F, G, T>(f: F, g: G) -> impl Fn(T) -> T
where
    F: Fn(T) -> T,
    G: Fn(T) -> T,
{
    move |x| g(f(x))
}

/// 分组函数
fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> HashMap<K, Vec<T>>
where
    K: std::hash::Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut groups = HashMap::new();
    for item in items {
        let key = key_fn(&item);
        groups.entry(key).or_insert_with(Vec::new).push(item);
    }
    groups
}

/// 高级函数式技巧演示
fn advanced_functional_demo() {
    // 惰性求值
    let lazy_iter = (0..1000000).map(|x| x * x).filter(|&x| x % 2 == 0).take(5);

    println!("  惰性求值 - 前5个偶数平方:");
    for val in lazy_iter {
        println!("    {}", val);
    }

    // 无限迭代器
    let fibonacci = Fibonacci::new();
    let first_ten: Vec<_> = fibonacci.take(10).collect();
    println!("  斐波那契数列前10项: {:?}", first_ten);

    // 迭代器的性能优化
    let large_vec: Vec<i32> = (0..100000).collect();

    let start = std::time::Instant::now();
    let _sum: i64 = large_vec.iter().map(|x| (*x as i64) * 2).sum();
    let duration = start.elapsed();
    println!("  迭代器版本耗时: {:?}", duration);

    let start = std::time::Instant::now();
    let mut _sum: i64 = 0;
    for i in &large_vec {
        _sum += (*i as i64) * 2;
    }
    let duration = start.elapsed();
    println!("  循环版本耗时: {:?}", duration); // 自定义迭代器
    let counter = Counter::new();
    let result: Vec<_> = counter.take(5).collect();
    println!("  自定义计数器迭代器: {:?}", result);

    // 函数式错误处理
    let numbers = vec!["1", "2", "not_a_number", "4"];
    let parsed: Result<Vec<i32>, _> = numbers.iter().map(|s| s.parse::<i32>()).collect();

    match parsed {
        Ok(nums) => println!("  解析成功: {:?}", nums),
        Err(e) => println!("  解析失败: {}", e),
    }

    // 只保留成功的解析结果
    let successful: Vec<i32> = numbers.iter().filter_map(|s| s.parse().ok()).collect();
    println!("  只保留成功解析: {:?}", successful);
}

/// 斐波那契迭代器
struct Fibonacci {
    current: usize,
    next: usize,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

/// 自定义计数器迭代器
struct Counter {
    current: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { current: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < 5 {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closures() {
        let add_one = |x| x + 1;
        assert_eq!(add_one(5), 6);

        let x = 4;
        let equal_to_x = |z| z == x;
        assert!(equal_to_x(4));
    }

    #[test]
    fn test_iterator_adaptors() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn test_filter() {
        let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let v2: Vec<_> = v1.into_iter().filter(|x| *x > 3).collect();
        assert_eq!(v2, vec![4, 5, 6]);
    }

    #[test]
    fn test_fold() {
        let v = vec![1, 2, 3, 4, 5];
        let sum = v.iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_find() {
        let v = vec![1, 2, 3, 4, 5];
        let found = v.iter().find(|&&x| x > 3);
        assert_eq!(found, Some(&4));
    }

    #[test]
    fn test_custom_iterator() {
        let counter = Counter::new();
        let result: Vec<_> = counter.collect();
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_fibonacci() {
        let fibonacci = Fibonacci::new();
        let first_five: Vec<_> = fibonacci.take(5).collect();
        assert_eq!(first_five, vec![0, 1, 1, 2, 3]);
    }

    #[test]
    fn test_function_composition() {
        let add_two = |x| x + 2;
        let multiply_three = |x| x * 3;
        let composed = compose(add_two, multiply_three);
        assert_eq!(composed(5), 21); // (5 + 2) * 3 = 21
    }

    #[test]
    fn test_group_by() {
        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
            Person {
                name: "Charlie".to_string(),
                age: 30,
            },
        ];

        let grouped = group_by(people, |person| person.age);
        assert_eq!(grouped.get(&30).unwrap().len(), 2);
        assert_eq!(grouped.get(&25).unwrap().len(), 1);
    }

    #[test]
    fn test_error_handling_with_iterators() {
        let numbers = vec!["1", "2", "3"];
        let parsed: Result<Vec<i32>, _> = numbers.iter().map(|s| s.parse::<i32>()).collect();

        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), vec![1, 2, 3]);
    }
}
