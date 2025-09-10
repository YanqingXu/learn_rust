//! 模式匹配学习模块
//!
//! 学习内容：
//! - match 表达式的深入使用
//! - if let 语法
//! - while let 语法
//! - 模式的类型和语法
//! - 模式匹配的实际应用

fn main() {
    println!("=== Rust 模式匹配学习 ===");

    // 1. 基本模式匹配
    println!("\n1. 基本模式匹配：");
    basic_pattern_matching();

    // 2. 模式的类型
    println!("\n2. 模式的类型：");
    pattern_types_demo();

    // 3. if let 语法
    println!("\n3. if let 语法：");
    if_let_demo();

    // 4. while let 语法
    println!("\n4. while let 语法：");
    while_let_demo();

    // 5. 函数参数中的模式
    println!("\n5. 函数参数中的模式：");
    function_parameters_demo();

    // 6. 复杂模式匹配
    println!("\n6. 复杂模式匹配：");
    complex_patterns_demo();
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
    // ... 更多州
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn basic_pattern_matching() {
    // 基本值的匹配
    let x = 1;

    match x {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("其他"),
    }

    // 硬币价值计算
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let quarter = Coin::Quarter(UsState::California);

    println!("硬币价值:");
    println!("  Penny: {} 美分", value_in_cents(penny));
    println!("  Nickel: {} 美分", value_in_cents(nickel));
    println!("  Quarter: {} 美分", value_in_cents(quarter));

    // Option 的匹配
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Option 匹配:");
    println!("  Some(5) + 1 = {six:?}");
    println!("  None + 1 = {none:?}");
}

fn pattern_types_demo() {
    // 字面量匹配
    let x = 1;
    match x {
        1 | 2 => println!("一或二"),
        3 => println!("三"),
        _ => println!("其他"),
    }

    // 范围匹配
    let y = 5;
    match y {
        1..=5 => println!("一到五"),
        6..=10 => println!("六到十"),
        _ => println!("其他范围"),
    }

    // 字符范围匹配
    let c = 'c';
    match c {
        'a'..='j' => println!("前十个字母"),
        'k'..='z' => println!("后面的字母"),
        _ => println!("其他字符"),
    }

    // 解构结构体
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("在 x 轴上，x = {x}"),
        Point { x: 0, y } => println!("在 y 轴上，y = {y}"),
        Point { x, y } => println!("不在轴上: ({x}, {y})"),
    }

    // 解构枚举
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移动到 ({x}, {y})"),
        Message::Write(text) => println!("写入: {text}"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("改变颜色为 RGB({r}, {g}, {b})")
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("改变颜色为 HSV({h}, {s}, {v})")
        }
    }
}

fn if_let_demo() {
    // 简单的 if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("最大值配置为: {max}");
    }

    // if let 与 else
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("使用你喜欢的颜色 {color} 作为背景");
    } else if is_tuesday {
        println!("周二是绿色的日子！");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("使用紫色作为背景颜色");
        } else {
            println!("使用橙色作为背景颜色");
        }
    } else {
        println!("使用蓝色作为背景颜色");
    }

    // 处理枚举变体
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println!("来自 {state:?} 州的25美分硬币！");
    } else {
        count += 1;
    }

    println!("非25美分硬币计数: {count}");
}

fn while_let_demo() {
    // while let 与 Vec
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("弹出栈元素:");
    while let Some(top) = stack.pop() {
        println!("  弹出: {top}");
    }

    // while let 与迭代器
    let v = vec!['a', 'b', 'c'];
    let mut iter = v.iter();

    println!("迭代器元素:");
    while let Some(value) = iter.next() {
        println!("  值: {value}");
    }

    // while let 与 Result
    let operations = vec!["42", "not_a_number", "17", "invalid"];

    println!("解析操作:");
    for op in operations {
        match op.parse::<i32>() {
            Ok(num) => println!("  成功解析: {num}"),
            Err(_) => println!("  解析失败: {op}"),
        }
    }
}

fn function_parameters_demo() {
    // 元组参数解构
    let point = (3, 5);
    print_coordinates(point);

    // 结构体参数解构
    let origin = Point { x: 0, y: 0 };
    print_point(&origin);

    // 引用和模式
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    println!("点的信息:");
    for point in &points {
        analyze_point(point);
    }
}

fn complex_patterns_demo() {
    // 守卫条件
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("小于五: {x}"),
        Some(x) => println!("大于等于五: {x}"),
        None => (),
    }

    // @ 绑定
    let msg = Message::Move { x: 1, y: 2 };

    match msg {
        Message::Move {
            x: x_val @ 1..=5,
            y: y_val @ 1..=5,
        } => {
            println!("在范围内移动到 ({x_val}, {y_val})");
        }
        Message::Move { x, y } => {
            println!("移动到 ({x}, {y})");
        }
        _ => {}
    }

    // 忽略值
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("一些数字: {first}, {third}, {fifth}");
        }
    }

    // 忽略结构体的部分字段
    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, .. } => println!("x 坐标是 {x}"),
    }

    // 嵌套的解构和匹配
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("脚: {feet}, 英寸: {inches}, 点: ({x}, {y})");
}

// 辅助函数
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自 {state:?} 州的25美分硬币!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_coordinates((x, y): (i32, i32)) {
    println!("当前位置: ({x}, {y})");
}

fn print_point(&Point { x, y }: &Point) {
    println!("点的坐标: ({x}, {y})");
}

fn analyze_point(point: &Point) {
    match point {
        Point { x: 0, y: 0 } => println!("  原点"),
        Point { x: 0, y } => println!("  在 y 轴上，y = {y}"),
        Point { x, y: 0 } => println!("  在 x 轴上，x = {x}"),
        Point { x, y } if x == y => println!("  在对角线上 ({x}, {y})"),
        Point { x, y } => println!("  任意点 ({x}, {y})"),
    }
}

// 实际应用示例：状态机
#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Processing { progress: u8 },
    Complete,
    Error(String),
}

fn process_state(state: State) -> State {
    match state {
        State::Idle => {
            println!("开始处理...");
            State::Processing { progress: 0 }
        }
        State::Processing { progress } if progress < 100 => {
            println!("处理中... {progress}%");
            State::Processing {
                progress: progress + 10,
            }
        }
        State::Processing { progress: 100 } => {
            println!("处理完成!");
            State::Complete
        }
        State::Processing { .. } => State::Error("进度超出范围".to_string()),
        State::Complete => {
            println!("已经完成");
            State::Complete
        }
        State::Error(msg) => {
            println!("错误状态: {msg}");
            State::Idle
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_in_cents() {
        assert_eq!(value_in_cents(Coin::Penny), 1);
        assert_eq!(value_in_cents(Coin::Nickel), 5);
        assert_eq!(value_in_cents(Coin::Dime), 10);
        assert_eq!(value_in_cents(Coin::Quarter(UsState::California)), 25);
    }

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(Some(5)), Some(6));
        assert_eq!(plus_one(None), None);
    }

    #[test]
    fn test_pattern_matching_ranges() {
        let test_range = |x| match x {
            1..=5 => "low",
            6..=10 => "medium",
            _ => "high",
        };

        assert_eq!(test_range(3), "low");
        assert_eq!(test_range(7), "medium");
        assert_eq!(test_range(15), "high");
    }

    #[test]
    fn test_point_matching() {
        let origin = Point { x: 0, y: 0 };
        let on_x_axis = Point { x: 5, y: 0 };
        let on_y_axis = Point { x: 0, y: 3 };
        let other = Point { x: 2, y: 3 };

        fn classify_point(p: &Point) -> &'static str {
            match p {
                Point { x: 0, y: 0 } => "origin",
                Point { x: 0, .. } => "y-axis",
                Point { y: 0, .. } => "x-axis",
                _ => "other",
            }
        }

        assert_eq!(classify_point(&origin), "origin");
        assert_eq!(classify_point(&on_x_axis), "x-axis");
        assert_eq!(classify_point(&on_y_axis), "y-axis");
        assert_eq!(classify_point(&other), "other");
    }

    #[test]
    fn test_state_machine() {
        let mut state = State::Idle;

        state = process_state(state);
        assert_eq!(state, State::Processing { progress: 0 });

        // 模拟处理过程
        while let State::Processing { progress } = state {
            state = process_state(state);
            if progress >= 90 {
                break;
            }
        }

        assert_eq!(state, State::Complete);
    }

    #[test]
    fn test_option_if_let() {
        let some_value = Some(10);
        let none_value: Option<i32> = None;

        let mut result = 0;

        if let Some(x) = some_value {
            result = x * 2;
        }

        assert_eq!(result, 20);

        if let Some(x) = none_value {
            result = x * 2;
        } else {
            result = -1;
        }

        assert_eq!(result, -1);
    }

    #[test]
    fn test_nested_matching() {
        let nested = Some(Some(5));
        let flat = Some(None);
        let nothing: Option<Option<i32>> = None;

        fn extract_inner(opt: Option<Option<i32>>) -> i32 {
            match opt {
                Some(Some(x)) => x,
                Some(None) => 0,
                None => -1,
            }
        }

        assert_eq!(extract_inner(nested), 5);
        assert_eq!(extract_inner(flat), 0);
        assert_eq!(extract_inner(nothing), -1);
    }
}
