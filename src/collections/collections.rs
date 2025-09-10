//! # Rust 集合类型学习
//!
//! 本模块演示 Rust 中常用的集合类型，包括：
//! - Vec<T> - 动态数组
//! - String - 可变字符串
//! - HashMap<K, V> - 哈希映射
//! - 其他集合类型
//!
//! 这些集合类型存储在堆上，可以在运行时增长或缩小

use std::collections::HashMap;

fn main() {
    println!("=== Rust 集合类型学习 ===\n");

    // 1. Vector 演示
    println!("1. Vector 动态数组：");
    vector_demo();
    println!();

    // 2. String 演示
    println!("2. String 字符串：");
    string_demo();
    println!();

    // 3. HashMap 演示
    println!("3. HashMap 哈希映射：");
    hashmap_demo();
    println!();

    // 4. 集合的高级用法
    println!("4. 集合的高级用法：");
    advanced_collections_demo();
}

/// Vector 动态数组演示
fn vector_demo() {
    // 创建 Vector
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("  空 Vector 添加元素: {:?}", v1);

    // 使用 vec! 宏创建
    let v2 = vec![1, 2, 3, 4, 5];
    println!("  使用 vec! 宏: {:?}", v2);

    // 访问元素
    let third: &i32 = &v2[2];
    println!("  第三个元素 (索引): {}", third);

    match v2.get(2) {
        Some(third) => println!("  第三个元素 (get方法): {}", third),
        None => println!("  没有第三个元素"),
    }

    // 遍历元素
    print!("  遍历所有元素: ");
    for i in &v2 {
        print!("{} ", i);
    }
    println!();

    // 修改元素
    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
    }
    println!("  修改后的 Vector: {:?}", v3);

    // Vector 存储不同类型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("  存储不同类型: {:?}", row);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

/// String 字符串演示
fn string_demo() {
    // 创建字符串
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("  空字符串添加内容: '{}'", s1);

    let s2 = "initial contents".to_string();
    println!("  从字面量创建: '{}'", s2);

    let s3 = String::from("Hello, world!");
    println!("  使用 String::from: '{}'", s3);

    // 更新字符串
    let mut s4 = String::from("foo");
    s4.push_str("bar");
    println!("  追加字符串: '{}'", s4);

    let mut s5 = String::from("lo");
    s5.push('l');
    println!("  追加字符: '{}'", s5);

    // 拼接字符串
    let s6 = String::from("Hello, ");
    let s7 = String::from("world!");
    let s8 = s6 + &s7; // s6 被移动，不能再使用
    println!("  使用 + 拼接: '{}'", s8);

    // 使用 format! 宏
    let s9 = String::from("tic");
    let s10 = String::from("tac");
    let s11 = String::from("toe");
    let s12 = format!("{}-{}-{}", s9, s10, s11);
    println!("  使用 format! 宏: '{}'", s12);

    // 字符串切片
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("  字符串切片 (字节): '{}'", s);

    // 遍历字符串
    print!("  遍历字符: ");
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!();

    print!("  遍历字节: ");
    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }
    println!();
}

/// HashMap 哈希映射演示
fn hashmap_demo() {
    // 创建 HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("  创建 HashMap: {:?}", scores);

    // 从 Vector 创建
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("  从 Vector 创建: {:?}", scores);

    // 访问值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("  Blue 队分数: {}", s),
        None => println!("  没有找到 Blue 队"),
    }

    // 遍历
    println!("  遍历所有键值对:");
    for (key, value) in &scores {
        println!("    {}: {}", key, value);
    }

    // 更新 HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // 覆盖值
    scores.insert(String::from("Blue"), 25);
    println!("  覆盖后: {:?}", scores);

    // 只在键没有对应值时插入
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("  使用 entry: {:?}", scores);

    // 根据旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("  单词计数: {:?}", map);
}

/// 集合的高级用法演示
fn advanced_collections_demo() {
    // Vector 的容量管理
    let mut v = Vec::with_capacity(10);
    println!("  Vector 容量: {}", v.capacity());
    v.push(1);
    v.push(2);
    v.push(3);
    println!("  添加 3 个元素后容量: {}", v.capacity());

    // Vector 的其他方法
    let mut v = vec![1, 2, 3, 4, 5];
    println!("  原始 Vector: {:?}", v);

    let popped = v.pop();
    println!("  pop() 结果: {:?}, Vector: {:?}", popped, v);

    v.insert(1, 10);
    println!("  insert(1, 10): {:?}", v);

    let removed = v.remove(2);
    println!("  remove(2) 结果: {}, Vector: {:?}", removed, v);

    // String 的其他方法
    let mut s = String::from("Hello, World!");
    println!("  原始字符串: '{}'", s);

    s.replace_range(7..12, "Rust");
    println!("  replace_range: '{}'", s);

    let words: Vec<&str> = s.split(", ").collect();
    println!("  split 结果: {:?}", words);

    // HashMap 的其他方法
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    println!("  HashMap 包含键 'a': {}", map.contains_key("a"));
    println!("  HashMap 长度: {}", map.len());

    let removed = map.remove("b");
    println!("  移除 'b': {:?}, HashMap: {:?}", removed, map);

    // 集合的转换
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("  Vector 映射: {:?} -> {:?}", v, doubled);

    let filtered: Vec<&i32> = v.iter().filter(|&&x| x > 2).collect();
    println!("  Vector 过滤: {:?}", filtered);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_operations() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v[3], 4);

        let popped = v.pop();
        assert_eq!(popped, Some(4));
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_string_operations() {
        let mut s = String::from("Hello");
        s.push_str(", world!");
        assert_eq!(s, "Hello, world!");

        let s2 = format!("{} {}", "Hello", "Rust");
        assert_eq!(s2, "Hello Rust");
    }

    #[test]
    fn test_hashmap_operations() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        assert_eq!(map.get("key1"), Some(&"value1"));
        assert_eq!(map.len(), 2);

        map.remove("key1");
        assert_eq!(map.get("key1"), None);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_collection_conversions() {
        let v = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);

        let even: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
        assert_eq!(even, vec![&2, &4]);
    }

    #[test]
    fn test_word_count() {
        let text = "hello world hello";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        assert_eq!(map.get("hello"), Some(&2));
        assert_eq!(map.get("world"), Some(&1));
    }

    #[test]
    fn test_vector_capacity() {
        let mut v = Vec::with_capacity(10);
        assert!(v.capacity() >= 10);

        for i in 0..5 {
            v.push(i);
        }
        assert_eq!(v.len(), 5);
        assert!(v.capacity() >= 10);
    }
}
