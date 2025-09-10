//! 简单计算器项目
//! 
//! 这是第一个实践项目，用于巩固基础语法学习。
//! 
//! 功能：
//! - 基本四则运算
//! - 用户输入处理
//! - 错误处理基础
//! - 循环和控制流

use std::io;

fn main() {
    println!("🧮 欢迎使用 Rust 计算器！");
    println!("支持的操作：+, -, *, /");
    println!("输入 'quit' 或 'q' 退出程序\n");
    
    loop {
        println!("请输入计算表达式（例如：5 + 3）：");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("读取输入失败");
        
        let input = input.trim();
        
        // 检查退出命令
        if input == "quit" || input == "q" {
            println!("再见！👋");
            break;
        }
        
        // 解析并计算表达式
        match parse_and_calculate(input) {
            Ok(result) => println!("结果：{}", result),
            Err(error) => println!("错误：{}", error),
        }
        
        println!(); // 空行分隔
    }
}

/// 解析输入并计算结果
fn parse_and_calculate(input: &str) -> Result<f64, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.len() != 3 {
        return Err("请输入格式：数字 操作符 数字（例如：5 + 3）".to_string());
    }
    
    // 解析第一个数字
    let num1 = parts[0].parse::<f64>()
        .map_err(|_| format!("'{}' 不是有效的数字", parts[0]))?;
    
    // 解析操作符
    let operator = parts[1];
    
    // 解析第二个数字
    let num2 = parts[2].parse::<f64>()
        .map_err(|_| format!("'{}' 不是有效的数字", parts[2]))?;
    
    // 执行计算
    calculate(num1, operator, num2)
}

/// 执行具体的计算操作
fn calculate(num1: f64, operator: &str, num2: f64) -> Result<f64, String> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("除数不能为零".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!("不支持的操作符：'{}'", operator)),
    }
}

/// 高级计算器功能（可选扩展）
#[allow(dead_code)]
mod advanced {
    use super::*;
    
    /// 支持更多操作的计算器
    pub fn advanced_calculate(num1: f64, operator: &str, num2: f64) -> Result<f64, String> {
        match operator {
            "+" => Ok(num1 + num2),
            "-" => Ok(num1 - num2),
            "*" => Ok(num1 * num2),
            "/" => {
                if num2 == 0.0 {
                    Err("除数不能为零".to_string())
                } else {
                    Ok(num1 / num2)
                }
            }
            "%" => {
                if num2 == 0.0 {
                    Err("除数不能为零".to_string())
                } else {
                    Ok(num1 % num2)
                }
            }
            "**" | "^" => Ok(num1.powf(num2)),
            _ => Err(format!("不支持的操作符：'{}'", operator)),
        }
    }
    
    /// 计算器历史记录
    pub struct CalculatorHistory {
        history: Vec<String>,
    }
    
    impl CalculatorHistory {
        pub fn new() -> Self {
            Self {
                history: Vec::new(),
            }
        }
        
        pub fn add_calculation(&mut self, expression: &str, result: f64) {
            let entry = format!("{} = {}", expression, result);
            self.history.push(entry);
        }
        
        pub fn show_history(&self) {
            if self.history.is_empty() {
                println!("暂无计算历史");
            } else {
                println!("计算历史：");
                for (i, entry) in self.history.iter().enumerate() {
                    println!("{}. {}", i + 1, entry);
                }
            }
        }
        
        pub fn clear_history(&mut self) {
            self.history.clear();
            println!("历史记录已清空");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_addition() {
        assert_eq!(calculate(2.0, "+", 3.0).unwrap(), 5.0);
        assert_eq!(calculate(-1.0, "+", 1.0).unwrap(), 0.0);
    }
    
    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(5.0, "-", 3.0).unwrap(), 2.0);
        assert_eq!(calculate(0.0, "-", 5.0).unwrap(), -5.0);
    }
    
    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(4.0, "*", 3.0).unwrap(), 12.0);
        assert_eq!(calculate(-2.0, "*", 3.0).unwrap(), -6.0);
    }
    
    #[test]
    fn test_division() {
        assert_eq!(calculate(6.0, "/", 2.0).unwrap(), 3.0);
        assert_eq!(calculate(7.0, "/", 2.0).unwrap(), 3.5);
    }
    
    #[test]
    fn test_division_by_zero() {
        assert!(calculate(5.0, "/", 0.0).is_err());
    }
    
    #[test]
    fn test_invalid_operator() {
        assert!(calculate(5.0, "&", 3.0).is_err());
    }
    
    #[test]
    fn test_parse_and_calculate() {
        assert_eq!(parse_and_calculate("5 + 3").unwrap(), 8.0);
        assert_eq!(parse_and_calculate("10 / 2").unwrap(), 5.0);
        assert!(parse_and_calculate("5 +").is_err());
        assert!(parse_and_calculate("abc + 3").is_err());
    }
    
    #[test]
    fn test_advanced_calculator() {
        use advanced::*;
        
        assert_eq!(advanced_calculate(2.0, "**", 3.0).unwrap(), 8.0);
        assert_eq!(advanced_calculate(10.0, "%", 3.0).unwrap(), 1.0);
    }
    
    #[test]
    fn test_calculator_history() {
        use advanced::*;
        
        let mut history = CalculatorHistory::new();
        history.add_calculation("5 + 3", 8.0);
        history.add_calculation("10 / 2", 5.0);
        
        // 这里只是测试不会 panic
        history.show_history();
        history.clear_history();
        history.show_history();
    }
}

/// 使用示例和学习要点
/// 
/// 这个计算器项目演示了以下 Rust 概念：
/// 
/// 1. **函数定义和调用**：
///    - main() 函数作为程序入口
///    - 自定义函数 parse_and_calculate() 和 calculate()
/// 
/// 2. **变量和可变性**：
///    - let mut input 声明可变变量
///    - 字符串的创建和修改
/// 
/// 3. **控制流**：
///    - loop 无限循环
///    - match 模式匹配
///    - if 条件判断
/// 
/// 4. **错误处理基础**：
///    - Result<T, E> 类型
///    - ? 操作符用于错误传播
///    - expect() 方法处理 panic
/// 
/// 5. **字符串处理**：
///    - String 和 &str 的区别
///    - split_whitespace() 分割字符串
///    - trim() 去除空白字符
/// 
/// 6. **集合类型**：
///    - Vec<T> 动态数组
///    - collect() 收集迭代器结果
/// 
/// 7. **类型转换**：
///    - parse() 方法将字符串转换为数字
///    - to_string() 将其他类型转换为字符串
/// 
/// 8. **模块系统**：
///    - mod advanced 子模块
///    - pub 关键字控制可见性
/// 
/// 9. **结构体和方法**：
///    - CalculatorHistory 结构体
///    - impl 块定义方法
/// 
/// 10. **测试**：
///     - #[cfg(test)] 测试模块
///     - #[test] 测试函数
///     - assert_eq! 和 assert! 宏