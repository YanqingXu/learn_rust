//! Result 和 Option 学习模块
//!
//! 学习内容：
//! - Option<T> 枚举的详细用法
//! - Result<T, E> 枚举的详细用法
//! - 组合器方法
//! - 错误转换和链式操作

fn main() {
    println!("=== Rust Result 和 Option 学习 ===");

    // 1. Option 枚举详解
    println!("\n1. Option 枚举详解：");
    option_details_demo();

    // 2. Result 枚举详解
    println!("\n2. Result 枚举详解：");
    result_details_demo();

    // 3. 组合器方法
    println!("\n3. 组合器方法：");
    combinator_methods_demo();

    // 4. 错误转换和链式操作
    println!("\n4. 错误转换和链式操作：");
    error_conversion_demo();

    // 5. 实际应用场景
    println!("\n5. 实际应用场景：");
    practical_applications_demo();
}

fn option_details_demo() {
    // Option 的创建
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    println!("Option 的基本使用:");
    println!("  Some(5): {some_number:?}");
    println!("  None: {no_number:?}");

    // Option 的方法
    println!("\nOption 的方法:");

    // is_some() 和 is_none()
    println!("  some_number.is_some(): {}", some_number.is_some());
    println!("  no_number.is_none(): {}", no_number.is_none());

    // unwrap_or() 和 unwrap_or_else()
    println!("  some_number.unwrap_or(0): {}", some_number.unwrap_or(0));
    println!("  no_number.unwrap_or(0): {}", no_number.unwrap_or(0));

    let default_value = no_number.unwrap_or_else(|| {
        println!("    计算默认值...");
        10
    });
    println!("  no_number.unwrap_or_else(): {default_value}");

    // map() 方法
    let doubled = some_number.map(|x| x * 2);
    println!("  some_number.map(|x| x * 2): {doubled:?}");

    let doubled_none = no_number.map(|x| x * 2);
    println!("  no_number.map(|x| x * 2): {doubled_none:?}");

    // and_then() 方法
    let result = some_number.and_then(|x| if x > 0 { Some(x * x) } else { None });
    println!("  some_number.and_then(square if positive): {result:?}");

    // filter() 方法
    let filtered = some_number.filter(|&x| x > 3);
    println!("  some_number.filter(|&x| x > 3): {filtered:?}");

    // take() 方法
    let mut option = Some(42);
    let taken = option.take();
    println!("  taken: {taken:?}, option after take: {option:?}");
}

fn result_details_demo() {
    // Result 的创建
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("Something went wrong");

    println!("Result 的基本使用:");
    println!("  Ok(42): {success:?}");
    println!("  Err: {failure:?}");

    // Result 的方法
    println!("\nResult 的方法:");

    // is_ok() 和 is_err()
    println!("  success.is_ok(): {}", success.is_ok());
    println!("  failure.is_err(): {}", failure.is_err());

    // unwrap_or() 和 unwrap_or_else()
    println!("  success.unwrap_or(0): {}", success.unwrap_or(0));
    println!("  failure.unwrap_or(0): {}", failure.unwrap_or(0));

    // map() 和 map_err()
    let doubled = success.map(|x| x * 2);
    println!("  success.map(|x| x * 2): {doubled:?}");

    let mapped_error = failure.map_err(|e| format!("Error: {e}"));
    println!("  failure.map_err(): {mapped_error:?}");

    // and_then()
    let chained = success.and_then(|x| if x > 40 { Ok(x + 10) } else { Err("Too small") });
    println!("  success.and_then(): {chained:?}");

    // or_else()
    let recovered: Result<i32, &str> = failure.or_else(|_| Ok(100));
    println!("  failure.or_else(|| Ok(100)): {recovered:?}");
}

fn combinator_methods_demo() {
    println!("组合器方法演示:");

    // Option 链式操作
    let numbers = vec![Some(1), None, Some(3), Some(4)];

    let processed: Vec<Option<i32>> = numbers.iter().map(|opt| opt.map(|x| x * 2)).collect();

    println!("  原始: {numbers:?}");
    println!("  翻倍: {processed:?}");

    // filter_map 移除 None 值
    let only_values: Vec<i32> = numbers.iter().filter_map(|&opt| opt).collect();

    println!("  过滤 None: {only_values:?}");

    // Option 的 zip 操作
    let opt1 = Some(1);
    let opt2 = Some(2);
    let opt3: Option<i32> = None;

    let zipped = opt1.zip(opt2);
    println!("  Some(1).zip(Some(2)): {zipped:?}");

    let zipped_none = opt1.zip(opt3);
    println!("  Some(1).zip(None): {zipped_none:?}");

    // Result 的链式操作
    let results = vec![Ok(1), Err("error"), Ok(3)];

    let processed_results: Vec<Result<i32, &str>> =
        results.iter().map(|res| res.map(|x| x * 2)).collect();

    println!("  Result 翻倍: {processed_results:?}");

    // collect() 用于 Result
    let all_ok = vec![Ok(1), Ok(2), Ok(3)];
    let collected: Result<Vec<i32>, &str> = all_ok.into_iter().collect();
    println!("  全部成功收集: {collected:?}");

    let with_error = vec![Ok(1), Err("error"), Ok(3)];
    let collected_error: Result<Vec<i32>, &str> = with_error.into_iter().collect();
    println!("  有错误的收集: {collected_error:?}");
}

fn error_conversion_demo() {
    println!("错误转换演示:");

    // 使用 ? 操作符进行错误传播
    match parse_numbers_and_sum(&["1", "2", "3"]) {
        Ok(sum) => println!("  数字总和: {sum}"),
        Err(e) => println!("  解析错误: {e}"),
    }

    match parse_numbers_and_sum(&["1", "invalid", "3"]) {
        Ok(sum) => println!("  数字总和: {sum}"),
        Err(e) => println!("  解析错误: {e}"),
    }

    // 复杂的错误转换
    match complex_operation("5", "10") {
        Ok(result) => println!("  复杂操作结果: {result}"),
        Err(e) => println!("  复杂操作错误: {e}"),
    }

    match complex_operation("invalid", "10") {
        Ok(result) => println!("  复杂操作结果: {result}"),
        Err(e) => println!("  复杂操作错误: {e}"),
    }

    // 多种错误类型的处理
    match multi_error_function(true) {
        Ok(value) => println!("  多错误函数成功: {value}"),
        Err(e) => println!("  多错误函数失败: {e:?}"),
    }

    match multi_error_function(false) {
        Ok(value) => println!("  多错误函数成功: {value}"),
        Err(e) => println!("  多错误函数失败: {e:?}"),
    }
}

fn practical_applications_demo() {
    println!("实际应用场景:");

    // 配置解析
    let config = parse_config(&["port=8080", "host=localhost", "debug=true"]);

    match config {
        Ok(cfg) => println!("  配置: {cfg:?}"),
        Err(e) => println!("  配置解析失败: {e}"),
    }

    // 数据库查询模拟
    let user_id = 123;
    match find_user(user_id) {
        Some(user) => println!("  找到用户: {user:?}"),
        None => println!("  用户 {user_id} 不存在"),
    }

    // 文件处理管道
    let files = vec!["file1.txt", "file2.txt", "nonexistent.txt"];

    for filename in files {
        match process_file(filename) {
            Ok(content) => println!("  {filename}: {content}"),
            Err(e) => println!("  {filename}: 错误 - {e}"),
        }
    }

    // 网络请求模拟
    let urls = vec!["http://api.example.com/users", "http://invalid.url"];

    for url in urls {
        match fetch_data(url) {
            Ok(data) => println!("  {url}: {data}"),
            Err(e) => println!("  {url}: {e}"),
        }
    }
}

// 辅助函数
fn parse_numbers_and_sum(strings: &[&str]) -> Result<i32, std::num::ParseIntError> {
    let mut sum = 0;
    for s in strings {
        sum += s.parse::<i32>()?;
    }
    Ok(sum)
}

fn complex_operation(s1: &str, s2: &str) -> Result<i32, String> {
    let num1 = s1
        .parse::<i32>()
        .map_err(|e| format!("解析第一个数字失败: {e}"))?;

    let num2 = s2
        .parse::<i32>()
        .map_err(|e| format!("解析第二个数字失败: {e}"))?;

    if num2 == 0 {
        return Err("第二个数字不能为零".to_string());
    }

    Ok(num1 / num2)
}

#[derive(Debug)]
enum MyError {
    ParseError(std::num::ParseIntError),
    DivisionByZero,
    NegativeNumber,
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::ParseError(error)
    }
}

fn multi_error_function(success: bool) -> Result<i32, MyError> {
    if !success {
        return Err(MyError::DivisionByZero);
    }

    let number = "42".parse::<i32>()?; // 自动转换错误类型

    if number < 0 {
        Err(MyError::NegativeNumber)
    } else {
        Ok(number)
    }
}

#[derive(Debug)]
struct Config {
    port: u16,
    host: String,
    debug: bool,
}

fn parse_config(lines: &[&str]) -> Result<Config, String> {
    let mut port = None;
    let mut host = None;
    let mut debug = None;

    for line in lines {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            return Err(format!("无效的配置行: {line}"));
        }

        match parts[0] {
            "port" => {
                port = Some(
                    parts[1]
                        .parse()
                        .map_err(|_| format!("无效的端口号: {}", parts[1]))?,
                )
            }
            "host" => host = Some(parts[1].to_string()),
            "debug" => {
                debug = Some(
                    parts[1]
                        .parse()
                        .map_err(|_| format!("无效的布尔值: {}", parts[1]))?,
                )
            }
            _ => return Err(format!("未知的配置项: {}", parts[0])),
        }
    }

    Ok(Config {
        port: port.ok_or("缺少端口配置")?,
        host: host.ok_or("缺少主机配置")?,
        debug: debug.ok_or("缺少调试配置")?,
    })
}

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

fn find_user(id: u32) -> Option<User> {
    // 模拟数据库查询
    if id == 123 {
        Some(User {
            id,
            name: "张三".to_string(),
        })
    } else {
        None
    }
}

fn process_file(filename: &str) -> Result<String, String> {
    // 模拟文件处理
    if filename.starts_with("nonexistent") {
        Err("文件不存在".to_string())
    } else {
        Ok(format!("文件 {filename} 的内容"))
    }
}

fn fetch_data(url: &str) -> Result<String, String> {
    // 模拟网络请求
    if url.contains("invalid") {
        Err("无效的 URL".to_string())
    } else {
        Ok(format!("来自 {url} 的数据"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_methods() {
        let some_value = Some(10);
        let none_value: Option<i32> = None;

        assert_eq!(some_value.unwrap_or(0), 10);
        assert_eq!(none_value.unwrap_or(0), 0);

        assert_eq!(some_value.map(|x| x * 2), Some(20));
        assert_eq!(none_value.map(|x| x * 2), None);
    }

    #[test]
    fn test_result_methods() {
        let ok_value: Result<i32, &str> = Ok(10);
        let err_value: Result<i32, &str> = Err("error");

        assert_eq!(ok_value.unwrap_or(0), 10);
        assert_eq!(err_value.unwrap_or(0), 0);

        assert_eq!(ok_value.map(|x| x * 2), Ok(20));
        assert!(err_value.map(|x| x * 2).is_err());
    }

    #[test]
    fn test_parse_numbers_and_sum() {
        assert_eq!(parse_numbers_and_sum(&["1", "2", "3"]), Ok(6));
        assert!(parse_numbers_and_sum(&["1", "invalid", "3"]).is_err());
    }

    #[test]
    fn test_complex_operation() {
        assert_eq!(complex_operation("10", "2"), Ok(5));
        assert!(complex_operation("invalid", "2").is_err());
        assert!(complex_operation("10", "0").is_err());
    }

    #[test]
    fn test_multi_error_function() {
        assert!(multi_error_function(true).is_ok());
        assert!(multi_error_function(false).is_err());
    }

    #[test]
    fn test_find_user() {
        assert!(find_user(123).is_some());
        assert!(find_user(999).is_none());
    }

    #[test]
    fn test_option_combinators() {
        let opt1 = Some(5);
        let opt2 = Some(10);
        let none_opt: Option<i32> = None;

        // zip
        assert_eq!(opt1.zip(opt2), Some((5, 10)));
        assert_eq!(opt1.zip(none_opt), None);

        // and_then
        let result = opt1.and_then(|x| Some(x * 2));
        assert_eq!(result, Some(10));

        // filter
        let filtered = opt1.filter(|&x| x > 3);
        assert_eq!(filtered, Some(5));

        let filtered_out = opt1.filter(|&x| x > 10);
        assert_eq!(filtered_out, None);
    }
}
