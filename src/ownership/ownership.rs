//! 所有权系统学习模块
//! 
//! 学习内容：
//! - 所有权规则
//! - 移动语义 (Move)
//! - 克隆 (Clone)
//! - 栈和堆的区别
//! - 函数调用中的所有权转移

fn main() {
    println!("=== Rust 所有权系统学习 ===");
    
    // 1. 基本所有权规则
    println!("\n1. 基本所有权规则：");
    basic_ownership_rules();
    
    // 2. 移动语义
    println!("\n2. 移动语义：");
    move_semantics();
    
    // 3. 克隆
    println!("\n3. 克隆：");
    cloning_demo();
    
    // 4. 栈和堆
    println!("\n4. 栈和堆的区别：");
    stack_and_heap();
    
    // 5. 函数和所有权
    println!("\n5. 函数和所有权：");
    functions_and_ownership();
    
    // 6. 返回值和所有权
    println!("\n6. 返回值和所有权：");
    return_values_and_ownership();
}

fn basic_ownership_rules() {
    // 规则1：Rust 中的每一个值都有一个被称为其所有者的变量
    let s1 = String::from("hello");
    println!("s1 拥有字符串: {}", s1);
    
    // 规则2：值在任一时刻有且只有一个所有者
    let s2 = s1; // s1 的所有权移动到 s2
    println!("s2 现在拥有字符串: {}", s2);
    // println!("{}", s1); // 编译错误！s1 不再有效
    
    // 规则3：当所有者离开作用域，这个值将被丢弃
    {
        let s3 = String::from("temporary");
        println!("s3 在内部作用域: {}", s3);
    } // s3 在这里被丢弃
    // println!("{}", s3); // 编译错误！s3 已经超出作用域
}

fn move_semantics() {
    // 对于存储在堆上的数据，赋值会发生移动
    let s1 = String::from("hello");
    let s2 = s1; // 移动发生
    
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // 编译错误！
    
    // 对于存储在栈上的数据，赋值会发生复制
    let x = 5;
    let y = x; // 复制发生
    
    println!("x: {}, y: {}", x, y); // 都可以使用
    
    // 实现了 Copy trait 的类型
    demonstrate_copy_types();
}

fn demonstrate_copy_types() {
    println!("\n  Copy trait 类型演示：");
    
    // 所有整数类型
    let a = 5;
    let b = a;
    println!("  整数 - a: {}, b: {}", a, b);
    
    // 布尔类型
    let flag1 = true;
    let flag2 = flag1;
    println!("  布尔 - flag1: {}, flag2: {}", flag1, flag2);
    
    // 浮点类型
    let f1 = 3.14;
    let f2 = f1;
    println!("  浮点 - f1: {}, f2: {}", f1, f2);
    
    // 字符类型
    let c1 = 'a';
    let c2 = c1;
    println!("  字符 - c1: {}, c2: {}", c1, c2);
    
    // 元组（如果所有元素都实现了 Copy）
    let tuple1 = (1, 2, 3);
    let tuple2 = tuple1;
    println!("  元组 - tuple1: {:?}, tuple2: {:?}", tuple1, tuple2);
}

fn cloning_demo() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝
    
    println!("s1: {}, s2: {}", s1, s2); // 都可以使用
    
    // 克隆的代价
    let large_string = "a".repeat(1000);
    let cloned = large_string.clone(); // 这会复制所有数据
    println!("原始字符串长度: {}", large_string.len());
    println!("克隆字符串长度: {}", cloned.len());
}

fn stack_and_heap() {
    // 栈上的数据：已知固定大小
    let stack_data = 42; // 存储在栈上
    println!("栈上的数据: {}", stack_data);
    
    // 堆上的数据：大小可变或编译时未知
    let heap_data = String::from("存储在堆上"); // 实际字符串数据在堆上
    println!("堆上的数据: {}", heap_data);
    
    // 数组 vs Vector
    let array = [1, 2, 3, 4, 5]; // 栈上
    let vector = vec![1, 2, 3, 4, 5]; // 堆上
    
    println!("数组（栈）: {:?}", array);
    println!("向量（堆）: {:?}", vector);
}

fn functions_and_ownership() {
    let s = String::from("hello");
    
    takes_ownership(s); // s 的值移动到函数里
    // println!("{}", s); // 编译错误！s 不再有效
    
    let x = 5;
    makes_copy(x); // x 被复制到函数里
    println!("x 仍然有效: {}", x); // x 仍然有效
}

fn takes_ownership(some_string: String) {
    println!("函数接收到: {}", some_string);
} // some_string 在这里被丢弃

fn makes_copy(some_integer: i32) {
    println!("函数接收到: {}", some_integer);
} // some_integer 在这里被丢弃，但没有特殊操作

fn return_values_and_ownership() {
    let s1 = gives_ownership(); // gives_ownership 将返回值移动给 s1
    println!("s1: {}", s1);
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 被移动到函数中，函数返回值移动给 s3
    
    println!("s3: {}", s3);
    // println!("{}", s2); // 编译错误！s2 已被移动
    
    // 如果想要函数使用一个值但不获取所有权怎么办？
    let s4 = String::from("world");
    let (s5, len) = calculate_length_with_ownership(s4);
    println!("字符串 '{}' 的长度是 {}。", s5, len);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 返回 some_string 并移动给调用函数
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回 a_string 并移动给调用函数
}

fn calculate_length_with_ownership(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // 返回字符串和长度
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_move_semantics() {
        let s1 = String::from("test");
        let s2 = s1;
        assert_eq!(s2, "test");
        // 注意：不能再使用 s1
    }
    
    #[test]
    fn test_copy_semantics() {
        let x = 5;
        let y = x;
        assert_eq!(x, 5);
        assert_eq!(y, 5);
    }
    
    #[test]
    fn test_clone() {
        let s1 = String::from("test");
        let s2 = s1.clone();
        assert_eq!(s1, "test");
        assert_eq!(s2, "test");
    }
}