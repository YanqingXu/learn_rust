//! 枚举学习模块
//! 
//! 学习内容：
//! - 枚举定义和使用
//! - Option 枚举
//! - Result 枚举
//! - 枚举方法

fn main() {
    println!("=== Rust 枚举学习 ===");
    
    // 1. 基本枚举
    println!("\n1. 基本枚举：");
    basic_enum_demo();
    
    // 2. 带数据的枚举
    println!("\n2. 带数据的枚举：");
    enum_with_data_demo();
    
    // 3. Option 枚举
    println!("\n3. Option 枚举：");
    option_enum_demo();
    
    // 4. Result 枚举
    println!("\n4. Result 枚举：");
    result_enum_demo();
    
    // 5. 枚举方法
    println!("\n5. 枚举方法：");
    enum_methods_demo();
}

// 基本枚举
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// 带数据的枚举
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// IP 地址枚举
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 状态枚举
#[derive(Debug, PartialEq)]
enum Status {
    Active,
    Inactive,
    Pending,
    Cancelled,
}

fn basic_enum_demo() {
    let north = Direction::North;
    let south = Direction::South;
    
    println!("方向: {north:?}, {south:?}");
    
    // 使用枚举
    move_in_direction(Direction::East);
    move_in_direction(Direction::West);
    
    // 枚举在 match 中的使用
    let directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    
    for direction in directions {
        let description = match direction {
            Direction::North => "向北",
            Direction::South => "向南",
            Direction::East => "向东",
            Direction::West => "向西",
        };
        println!("移动方向: {description}");
    }
}

fn enum_with_data_demo() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("Hello"));
    let m4 = Message::ChangeColor(255, 0, 0);
    
    println!("消息列表:");
    process_message(m1);
    process_message(m2);
    process_message(m3);
    process_message(m4);
    
    // IP 地址示例
    let ipv4 = IpAddr::V4(127, 0, 0, 1);
    let ipv6 = IpAddr::V6(String::from("::1"));
    
    println!("\nIP 地址:");
    print_ip_address(ipv4);
    print_ip_address(ipv6);
    
    // 状态示例
    let current_status = Status::Active;
    println!("\n当前状态: {current_status:?}");
    
    if current_status == Status::Active {
        println!("系统正在运行");
    }
}

fn option_enum_demo() {
    // Option 是 Rust 标准库中的枚举
    let some_number = Some(5);
    let some_string = Some(String::from("hello"));
    let no_number: Option<i32> = None;
    
    println!("Option 示例:");
    print_option_number(some_number);
    print_option_number(no_number);
    
    // Option 的实际使用
    let numbers = vec![1, 2, 3, 4, 5];
    
    match find_number(&numbers, 3) {
        Some(index) => println!("找到数字 3 在索引 {index}"),
        None => println!("未找到数字 3"),
    }
    
    match find_number(&numbers, 10) {
        Some(index) => println!("找到数字 10 在索引 {index}"),
        None => println!("未找到数字 10"),
    }
    
    // 使用 if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("最大值配置为: {max}");
    }
    
    // 使用 unwrap_or
    let default_value = no_number.unwrap_or(0);
    println!("带默认值: {default_value}");
}

fn result_enum_demo() {
    // Result 用于可能失败的操作
    let good_result = divide(10.0, 2.0);
    let bad_result = divide(10.0, 0.0);
    
    match good_result {
        Ok(value) => println!("除法结果: {value}"),
        Err(error) => println!("除法错误: {error}"),
    }
    
    match bad_result {
        Ok(value) => println!("除法结果: {value}"),
        Err(error) => println!("除法错误: {error}"),
    }
    
    // 使用 ? 操作符
    match parse_and_double("10") {
        Ok(result) => println!("解析并翻倍: {result}"),
        Err(e) => println!("解析错误: {e}"),
    }
    
    match parse_and_double("abc") {
        Ok(result) => println!("解析并翻倍: {result}"),
        Err(e) => println!("解析错误: {e}"),
    }
    
    // 多种错误类型
    let file_result = read_file_size("example.txt");
    match file_result {
        Ok(size) => println!("文件大小: {size} 字节"),
        Err(error) => println!("读取文件错误: {error:?}"),
    }
}

fn enum_methods_demo() {
    let message = Message::Write(String::from("Hello, Rust!"));
    
    // 调用枚举的方法
    message.process();
    
    let length = message.length();
    println!("消息长度: {length}");
    
    // 状态转换
    let mut status = Status::Pending;
    println!("初始状态: {status:?}");
    
    status = status.next();
    println!("下一个状态: {status:?}");
    
    // 枚举的实用方法
    let direction = Direction::North;
    println!("方向的相对方向: {:?}", direction.opposite());
    println!("方向是否为水平: {}", direction.is_horizontal());
}

// 辅助函数
fn move_in_direction(direction: Direction) {
    match direction {
        Direction::North => println!("向北移动"),
        Direction::South => println!("向南移动"),
        Direction::East => println!("向东移动"),
        Direction::West => println!("向西移动"),
    }
}

fn process_message(message: Message) {
    match message {
        Message::Quit => println!("  退出消息"),
        Message::Move { x, y } => println!("  移动到坐标 ({x}, {y})"),
        Message::Write(text) => println!("  写入文本: {text}"),
        Message::ChangeColor(r, g, b) => println!("  改变颜色为 RGB({r}, {g}, {b})"),
    }
}

fn print_ip_address(ip: IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => println!("  IPv4: {a}.{b}.{c}.{d}"),
        IpAddr::V6(addr) => println!("  IPv6: {addr}"),
    }
}

fn print_option_number(option: Option<i32>) {
    match option {
        Some(num) => println!("  数字: {num}"),
        None => println!("  没有数字"),
    }
}

fn find_number(numbers: &[i32], target: i32) -> Option<usize> {
    for (index, &number) in numbers.iter().enumerate() {
        if number == target {
            return Some(index);
        }
    }
    None
}

fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(x / y)
    }
}

fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let number = s.parse::<i32>()?;
    Ok(number * 2)
}

#[derive(Debug)]
enum FileError {
    NotFound,
    PermissionDenied,
    InvalidFormat,
}

fn read_file_size(_filename: &str) -> Result<u64, FileError> {
    // 模拟文件读取
    Err(FileError::NotFound)
}

// 为 Message 实现方法
impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("  处理退出请求"),
            Message::Move { x, y } => println!("  处理移动到 ({x}, {y})"),
            Message::Write(text) => println!("  处理写入: {text}"),
            Message::ChangeColor(r, g, b) => println!("  处理颜色变更: RGB({r}, {g}, {b})"),
        }
    }
    
    fn length(&self) -> usize {
        match self {
            Message::Quit => 0,
            Message::Move { .. } => 2, // x, y 两个坐标
            Message::Write(text) => text.len(),
            Message::ChangeColor(_, _, _) => 3, // r, g, b 三个值
        }
    }
}

// 为 Direction 实现方法
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    
    fn is_horizontal(&self) -> bool {
        matches!(self, Direction::East | Direction::West)
    }
    
    fn is_vertical(&self) -> bool {
        matches!(self, Direction::North | Direction::South)
    }
}

// 为 Status 实现方法
impl Status {
    fn next(&self) -> Status {
        match self {
            Status::Pending => Status::Active,
            Status::Active => Status::Inactive,
            Status::Inactive => Status::Cancelled,
            Status::Cancelled => Status::Pending,
        }
    }
    
    fn is_active(&self) -> bool {
        *self == Status::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_direction_opposite() {
        assert_eq!(Direction::North.opposite() as u8, Direction::South as u8);
        assert_eq!(Direction::East.opposite() as u8, Direction::West as u8);
    }
    
    #[test]
    fn test_direction_horizontal() {
        assert!(Direction::East.is_horizontal());
        assert!(Direction::West.is_horizontal());
        assert!(!Direction::North.is_horizontal());
        assert!(!Direction::South.is_horizontal());
    }
    
    #[test]
    fn test_status_next() {
        assert_eq!(Status::Pending.next(), Status::Active);
        assert_eq!(Status::Active.next(), Status::Inactive);
        assert_eq!(Status::Inactive.next(), Status::Cancelled);
        assert_eq!(Status::Cancelled.next(), Status::Pending);
    }
    
    #[test]
    fn test_status_is_active() {
        assert!(Status::Active.is_active());
        assert!(!Status::Pending.is_active());
        assert!(!Status::Inactive.is_active());
        assert!(!Status::Cancelled.is_active());
    }
    
    #[test]
    fn test_find_number() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(find_number(&numbers, 3), Some(2));
        assert_eq!(find_number(&numbers, 10), None);
    }
    
    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert!(divide(10.0, 0.0).is_err());
    }
    
    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("5"), Ok(10));
        assert!(parse_and_double("abc").is_err());
    }
    
    #[test]
    fn test_message_length() {
        let quit = Message::Quit;
        let move_msg = Message::Move { x: 1, y: 2 };
        let write_msg = Message::Write(String::from("hello"));
        let color_msg = Message::ChangeColor(255, 0, 0);
        
        assert_eq!(quit.length(), 0);
        assert_eq!(move_msg.length(), 2);
        assert_eq!(write_msg.length(), 5);
        assert_eq!(color_msg.length(), 3);
    }
    
    #[test]
    fn test_option_operations() {
        let some_value = Some(5);
        let no_value: Option<i32> = None;
        
        assert_eq!(some_value.unwrap_or(0), 5);
        assert_eq!(no_value.unwrap_or(0), 0);
        
        assert!(some_value.is_some());
        assert!(no_value.is_none());
    }
}
