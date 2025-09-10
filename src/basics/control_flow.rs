//! 控制流学习模块
//!
//! 学习内容：
//! - if 表达式
//! - loop 循环
//! - while 循环  
//! - for 循环
//! - match 表达式
//! - 循环控制（break, continue）

fn main() {
    println!("=== Rust 控制流学习 ===");

    // 1. if 表达式
    println!("\n1. if 表达式：");
    if_expressions_demo();

    // 2. loop 循环
    println!("\n2. loop 循环：");
    loop_demo();

    // 3. while 循环
    println!("\n3. while 循环：");
    while_demo();

    // 4. for 循环
    println!("\n4. for 循环：");
    for_demo();

    // 5. match 表达式
    println!("\n5. match 表达式：");
    match_demo();

    // 6. 嵌套循环和标签
    println!("\n6. 嵌套循环和标签：");
    nested_loops_demo();
}

fn if_expressions_demo() {
    // 基本 if
    let number = 6;

    if number % 4 == 0 {
        println!("数字能被 4 整除");
    } else if number % 3 == 0 {
        println!("数字能被 3 整除");
    } else if number % 2 == 0 {
        println!("数字能被 2 整除");
    } else {
        println!("数字不能被 4、3、2 整除");
    }

    // if 作为表达式
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("条件表达式的值：{}", number);

    // 复杂条件
    let age = 25;
    let has_license = true;

    if age >= 18 && has_license {
        println!("可以开车");
    } else if age >= 18 {
        println!("需要先获得驾照");
    } else {
        println!("年龄不够");
    }
}

fn loop_demo() {
    println!("使用 loop 计算平方:");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // 从 loop 返回值
        }

        print!("{} ", counter * counter);
    };

    println!("\n循环结束，返回值：{}", result);
}

fn while_demo() {
    println!("使用 while 倒计时:");

    let mut number = 5;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("发射！🚀");

    // while let 模式匹配
    let mut stack = vec![1, 2, 3];

    println!("\n使用 while let 弹出栈元素:");
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
}

fn for_demo() {
    // 遍历数组
    let a = [10, 20, 30, 40, 50];

    println!("遍历数组:");
    for element in a {
        println!("值: {}", element);
    }

    // 使用范围
    println!("\n使用范围 (1..4):");
    for number in 1..4 {
        println!("数字: {}", number);
    }

    // 包含结束值的范围
    println!("\n使用范围 (1..=4):");
    for number in 1..=4 {
        println!("数字: {}", number);
    }

    // 反向遍历
    println!("\n反向遍历:");
    for number in (1..4).rev() {
        println!("数字: {}", number);
    }

    // 带索引的遍历
    let names = ["Alice", "Bob", "Charlie"];
    println!("\n带索引的遍历:");
    for (index, name) in names.iter().enumerate() {
        println!("{}: {}", index, name);
    }
}

fn match_demo() {
    let number = 13;

    // 基本 match
    println!("基本 match:");
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 | 11 => println!("这是一个小质数"),
        13..=19 => println!("十几"),
        _ => println!("其他数字"),
    }

    // match 绑定值
    let x = Some(5);

    match x {
        Some(i) => println!("匹配到 Some，值为：{}", i),
        None => println!("匹配到 None"),
    }

    // match 守卫
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("小于五：{}", x),
        Some(x) => println!("等于或大于五：{}", x),
        None => (),
    }

    // 解构元组
    let point = (3, 5);

    match point {
        (0, y) => println!("在 y 轴上，y = {}", y),
        (x, 0) => println!("在 x 轴上，x = {}", x),
        (x, y) => println!("不在轴上：({}, {})", x, y),
    }
}

fn nested_loops_demo() {
    println!("嵌套循环和循环标签:");

    'outer: loop {
        println!("进入外层循环");

        'inner: loop {
            println!("  进入内层循环");

            // 这会跳出内层循环
            break;

            // 这会跳出外层循环
            // break 'outer;
        }

        println!("这行会被执行");
        break 'outer;
    }

    println!("跳出外层循环");

    // 实际例子：查找数字
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let target = 5;

    'search: for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == target {
                println!("找到 {} 在位置 ({}, {})", target, i, j);
                break 'search;
            }
        }
    }
}

// 辅助函数：判断数字类型
fn classify_number(n: i32) -> &'static str {
    match n {
        n if n < 0 => "负数",
        0 => "零",
        1..=10 => "小正数",
        11..=100 => "中等正数",
        _ => "大正数",
    }
}

// 辅助函数：计算阶乘
fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

// 辅助函数：FizzBuzz 游戏
fn fizz_buzz(limit: i32) {
    println!("\nFizzBuzz 游戏 (1 到 {}):", limit);

    for i in 1..=limit {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_number() {
        assert_eq!(classify_number(-5), "负数");
        assert_eq!(classify_number(0), "零");
        assert_eq!(classify_number(5), "小正数");
        assert_eq!(classify_number(50), "中等正数");
        assert_eq!(classify_number(150), "大正数");
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_control_flow() {
        // 测试 if 表达式
        let result = if true { 1 } else { 0 };
        assert_eq!(result, 1);

        // 测试 match 表达式
        let x = 5;
        let result = match x {
            1..=5 => "小数",
            _ => "大数",
        };
        assert_eq!(result, "小数");
    }

    #[test]
    fn test_loop_break_value() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 3 {
                break counter * 10;
            }
        };
        assert_eq!(result, 30);
    }
}

// 学习要点总结：
//
// 1. **if 表达式**：
//    - if 是表达式，可以返回值
//    - 所有分支必须返回相同类型
//    - 条件必须是 bool 类型
//
// 2. **循环类型**：
//    - loop：无限循环，需要 break 退出
//    - while：条件循环
//    - for：迭代循环，最常用
//
// 3. **match 表达式**：
//    - 必须覆盖所有可能的情况
//    - 使用 _ 匹配其他所有情况
//    - 支持范围匹配和守卫条件
//
// 4. **循环控制**：
//    - break：跳出循环，可以带返回值
//    - continue：跳过本次迭代
//    - 循环标签：控制嵌套循环
//
// 5. **模式匹配**：
//    - 可以解构复杂数据类型
//    - 支持变量绑定
//    - 守卫条件提供额外判断
