//! 函数学习模块
//! 
//! 学习内容：
//! - 函数定义和调用
//! - 参数和返回值
//! - 表达式和语句
//! - 函数指针
//! - 闭包基础

fn main() {
    println!("=== Rust 函数学习 ===");
    
    // 1. 基本函数调用
    println!("\n1. 基本函数：");
    greet();
    greet_person("Alice");
    
    // 2. 有返回值的函数
    println!("\n2. 有返回值的函数：");
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    let product = multiply(4, 6);
    println!("4 × 6 = {}", product);
    
    // 3. 返回多个值
    println!("\n3. 返回多个值：");
    let (sum, diff) = add_and_subtract(10, 3);
    println!("10 + 3 = {}, 10 - 3 = {}", sum, diff);
    
    // 4. 表达式和语句
    println!("\n4. 表达式和语句：");
    let y = {
        let x = 3;
        x + 1  // 这是表达式，没有分号
    };
    println!("表达式的值: {}", y);
    
    // 5. 提前返回
    println!("\n5. 提前返回：");
    println!("绝对值 -5: {}", absolute_value(-5));
    println!("绝对值 3: {}", absolute_value(3));
    
    // 6. 函数指针
    println!("\n6. 函数指针：");
    let operation = add;
    println!("通过函数指针调用: {}", operation(2, 3));
    
    // 7. 高阶函数
    println!("\n7. 高阶函数：");
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = apply_to_all(numbers, double);
    println!("翻倍后的数组: {:?}", doubled);
    
    // 8. 递归函数
    println!("\n8. 递归函数：");
    println!("5 的阶乘: {}", factorial(5));
    println!("斐波那契数列第 10 项: {}", fibonacci(10));
    
    // 9. 方法语法预览
    println!("\n9. 方法语法预览：");
    let rect = Rectangle { width: 30, height: 50 };
    println!("矩形面积: {}", rect.area());
    println!("矩形周长: {}", rect.perimeter());
}

// 无参数无返回值
fn greet() {
    println!("Hello, World!");
}

// 有参数无返回值
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// 有参数有返回值
fn add(a: i32, b: i32) -> i32 {
    a + b  // 表达式，没有分号
}

// 显式使用 return
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // 使用 return 关键字
}

// 返回元组（多个值）
fn add_and_subtract(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}

// 提前返回示例
fn absolute_value(x: i32) -> i32 {
    if x < 0 {
        return -x;  // 提前返回
    }
    x  // 正常返回
}

// 辅助函数
fn double(x: i32) -> i32 {
    x * 2
}

// 高阶函数：接受函数作为参数
fn apply_to_all(numbers: Vec<i32>, func: fn(i32) -> i32) -> Vec<i32> {
    numbers.into_iter().map(func).collect()
}

// 递归函数：计算阶乘
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// 递归函数：斐波那契数列
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 简单的结构体用于演示方法
struct Rectangle {
    width: u32,
    height: u32,
}

// 为结构体实现方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }
    
    #[test]
    fn test_rectangle() {
        let rect = Rectangle { width: 3, height: 4 };
        assert_eq!(rect.area(), 12);
        assert_eq!(rect.perimeter(), 14);
    }
}