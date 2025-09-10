//! 错误处理学习模块
//!
//! 学习内容：
//! - panic! 宏和不可恢复错误
//! - Result<T, E> 和可恢复错误
//! - ? 操作符
//! - 自定义错误类型
//! - 错误传播

use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("=== Rust 错误处理学习 ===");

    // 1. 不可恢复错误 (panic!)
    println!("\n1. 不可恢复错误：");
    unrecoverable_errors_demo();

    // 2. 可恢复错误 (Result)
    println!("\n2. 可恢复错误：");
    recoverable_errors_demo();

    // 3. ? 操作符
    println!("\n3. ? 操作符：");
    question_mark_operator_demo();

    // 4. 自定义错误类型
    println!("\n4. 自定义错误类型：");
    custom_error_types_demo();

    // 5. 错误处理的最佳实践
    println!("\n5. 错误处理的最佳实践：");
    best_practices_demo();
}

fn unrecoverable_errors_demo() {
    // panic! 宏会导致程序崩溃
    println!("演示 panic! 的使用场景（注释掉以避免程序崩溃）");

    // panic!("程序崩溃了！");

    // 数组越界会触发 panic
    // let v = vec![1, 2, 3];
    // v[99];  // 这会导致 panic

    // 使用 unwrap() 在 None 或 Err 时会 panic
    let some_option: Option<i32> = None;
    // some_option.unwrap();  // 这会 panic

    // 更安全的替代方案
    match some_option {
        Some(value) => println!("值: {value}"),
        None => println!("没有值，避免了 panic"),
    }

    // expect() 提供更好的错误信息
    let some_result: Result<i32, &str> = Err("出错了");
    match some_result {
        Ok(value) => println!("成功: {value}"),
        Err(e) => println!("错误: {e}"),
    }
}

fn recoverable_errors_demo() {
    // 文件操作可能失败
    let filename = "hello.txt";

    match File::open(filename) {
        Ok(file) => println!("成功打开文件: {file:?}"),
        Err(error) => println!("打开文件失败: {error}"),
    }

    // 使用 unwrap_or_else 处理错误
    let file = File::open(filename).unwrap_or_else(|error| {
        println!("创建新文件，因为: {error}");
        File::create(filename).unwrap_or_else(|error| {
            panic!("创建文件也失败了: {error:?}");
        })
    });

    println!("文件处理完成: {file:?}");

    // 数字解析示例
    let number_str = "42";
    match number_str.parse::<i32>() {
        Ok(number) => println!("解析成功: {number}"),
        Err(error) => println!("解析失败: {error}"),
    }

    let invalid_str = "not_a_number";
    match invalid_str.parse::<i32>() {
        Ok(number) => println!("解析成功: {number}"),
        Err(error) => println!("解析失败: {error}"),
    }
}

fn question_mark_operator_demo() {
    // 使用 ? 操作符简化错误处理
    match read_username_from_file("username.txt") {
        Ok(username) => println!("用户名: {username}"),
        Err(error) => println!("读取用户名失败: {error}"),
    }

    // 链式操作的错误处理
    match parse_and_double("25") {
        Ok(result) => println!("解析并翻倍: {result}"),
        Err(error) => println!("操作失败: {error}"),
    }

    match parse_and_double("invalid") {
        Ok(result) => println!("解析并翻倍: {result}"),
        Err(error) => println!("操作失败: {error}"),
    }

    // Option 的 ? 操作符
    match get_first_and_last(&[1, 2, 3, 4, 5]) {
        Some((first, last)) => println!("首尾元素: {first}, {last}"),
        None => println!("数组为空或只有一个元素"),
    }
}

fn custom_error_types_demo() {
    // 使用自定义错误类型
    match divide_numbers(10.0, 2.0) {
        Ok(result) => println!("除法结果: {result}"),
        Err(error) => println!("除法错误: {error}"),
    }

    match divide_numbers(10.0, 0.0) {
        Ok(result) => println!("除法结果: {result}"),
        Err(error) => println!("除法错误: {error}"),
    }

    // 复合错误类型示例
    match process_user_data("25", "john@example.com") {
        Ok(user) => println!("用户: {user:?}"),
        Err(error) => println!("处理用户数据失败: {error}"),
    }

    match process_user_data("invalid_age", "invalid_email") {
        Ok(user) => println!("用户: {user:?}"),
        Err(error) => println!("处理用户数据失败: {error}"),
    }
}

fn best_practices_demo() {
    println!("错误处理最佳实践:");

    // 1. 使用 Result 而不是 panic!
    println!("1. 优先使用 Result 类型");

    // 2. 提供有意义的错误信息
    match validate_email("invalid_email") {
        Ok(email) => println!("有效邮箱: {email}"),
        Err(error) => println!("邮箱验证失败: {error}"),
    }

    // 3. 错误的分层处理
    match high_level_operation() {
        Ok(result) => println!("高级操作成功: {result}"),
        Err(error) => println!("高级操作失败: {error}"),
    }

    // 4. 使用类型系统确保错误处理
    let safe_result = safe_division(10, 2);
    println!("安全除法: {safe_result:?}");

    let safe_result = safe_division(10, 0);
    println!("安全除法: {safe_result:?}");
}

// 使用 ? 操作符的函数
fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let mut username_file = File::open(filename)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// 更简洁的版本
fn read_username_from_file_v2(filename: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(filename)?.read_to_string(&mut username)?;
    Ok(username)
}

// 链式操作
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let number = s.parse::<i32>()?;
    Ok(number * 2)
}

// Option 的 ? 操作符
fn get_first_and_last(slice: &[i32]) -> Option<(i32, i32)> {
    let first = slice.first()?;
    let last = slice.last()?;
    if slice.len() > 1 {
        Some((*first, *last))
    } else {
        None
    }
}

// 自定义错误类型
#[derive(Debug, PartialEq)]
enum DivisionError {
    DivisionByZero,
    InvalidInput,
}

impl std::fmt::Display for DivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DivisionError::DivisionByZero => write!(f, "除数不能为零"),
            DivisionError::InvalidInput => write!(f, "输入无效"),
        }
    }
}

impl std::error::Error for DivisionError {}

fn divide_numbers(x: f64, y: f64) -> Result<f64, DivisionError> {
    if y == 0.0 {
        Err(DivisionError::DivisionByZero)
    } else if x.is_nan() || y.is_nan() {
        Err(DivisionError::InvalidInput)
    } else {
        Ok(x / y)
    }
}

// 复合错误类型
#[derive(Debug)]
struct User {
    age: u32,
    email: String,
}

#[derive(Debug)]
enum UserError {
    InvalidAge(std::num::ParseIntError),
    InvalidEmail(String),
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserError::InvalidAge(e) => write!(f, "年龄解析错误: {e}"),
            UserError::InvalidEmail(email) => write!(f, "邮箱格式错误: {email}"),
        }
    }
}

impl std::error::Error for UserError {}

impl From<std::num::ParseIntError> for UserError {
    fn from(error: std::num::ParseIntError) -> Self {
        UserError::InvalidAge(error)
    }
}

fn process_user_data(age_str: &str, email: &str) -> Result<User, UserError> {
    let age = age_str.parse::<u32>()?; // 自动转换 ParseIntError

    if !email.contains('@') {
        return Err(UserError::InvalidEmail(email.to_string()));
    }

    Ok(User {
        age,
        email: email.to_string(),
    })
}

// 邮箱验证
fn validate_email(email: &str) -> Result<&str, String> {
    if email.contains('@') && email.contains('.') {
        Ok(email)
    } else {
        Err(format!("无效的邮箱格式: {email}"))
    }
}

// 分层错误处理
fn low_level_operation() -> Result<i32, &'static str> {
    // 模拟可能失败的低级操作
    Err("低级操作失败")
}

fn mid_level_operation() -> Result<i32, String> {
    match low_level_operation() {
        Ok(value) => Ok(value * 2),
        Err(e) => Err(format!("中级操作失败，原因: {e}")),
    }
}

fn high_level_operation() -> Result<String, String> {
    match mid_level_operation() {
        Ok(value) => Ok(format!("高级操作成功，结果: {value}")),
        Err(e) => Err(format!("高级操作失败，原因: {e}")),
    }
}

// 使用类型系统避免错误
fn safe_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("5"), Ok(10));
        assert!(parse_and_double("invalid").is_err());
    }

    #[test]
    fn test_divide_numbers() {
        assert_eq!(divide_numbers(10.0, 2.0), Ok(5.0));
        assert!(matches!(
            divide_numbers(10.0, 0.0),
            Err(DivisionError::DivisionByZero)
        ));
    }

    #[test]
    fn test_get_first_and_last() {
        assert_eq!(get_first_and_last(&[1, 2, 3]), Some((1, 3)));
        assert_eq!(get_first_and_last(&[1]), None);
        assert_eq!(get_first_and_last(&[]), None);
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid").is_err());
    }

    #[test]
    fn test_process_user_data() {
        assert!(process_user_data("25", "user@example.com").is_ok());
        assert!(process_user_data("invalid", "user@example.com").is_err());
        assert!(process_user_data("25", "invalid").is_err());
    }

    #[test]
    fn test_safe_division() {
        assert_eq!(safe_division(10, 2), Some(5));
        assert_eq!(safe_division(10, 0), None);
    }

    #[test]
    fn test_error_propagation() {
        // 测试错误传播
        assert!(high_level_operation().is_err());
    }

    #[test]
    fn test_custom_error_display() {
        let error = DivisionError::DivisionByZero;
        assert_eq!(format!("{error}"), "除数不能为零");

        let error = UserError::InvalidEmail("test".to_string());
        assert!(format!("{error}").contains("邮箱格式错误"));
    }
}
