//! 数据类型学习模块
//!
//! 学习内容：
//! - 标量类型（整数、浮点数、布尔值、字符）
//! - 复合类型（元组、数组）
//! - 类型推断和类型注解
//! - 类型转换

fn main() {
    println!("=== Rust 数据类型学习 ===");

    // 1. 标量类型
    println!("\n1. 标量类型：");
    scalar_types_demo();

    // 2. 复合类型
    println!("\n2. 复合类型：");
    compound_types_demo();

    // 3. 类型推断和注解
    println!("\n3. 类型推断和注解：");
    type_inference_demo();

    // 4. 类型转换
    println!("\n4. 类型转换：");
    type_conversion_demo();

    // 5. 数值操作
    println!("\n5. 数值操作：");
    numeric_operations_demo();

    // 6. 字符和字符串
    println!("\n6. 字符和字符串：");
    char_and_string_demo();
}

fn scalar_types_demo() {
    // 整数类型
    println!("整数类型：");
    let decimal = 98_222; // 十进制
    let hex = 0xff; // 十六进制
    let octal = 0o77; // 八进制
    let binary = 0b1111_0000; // 二进制
    let byte = b'A'; // 字节（仅限 u8）

    println!("  十进制: {decimal}");
    println!("  十六进制: {hex}");
    println!("  八进制: {octal}");
    println!("  二进制: {binary}");
    println!("  字节: {byte}");

    // 不同大小的整数
    let small: i8 = 127; // -128 到 127
    let medium: i16 = 32767; // -32,768 到 32,767
    let standard: i32 = 2147483647; // 默认整数类型
    let large: i64 = 9223372036854775807;
    let huge: i128 = 170141183460469231731687303715884105727;

    println!("  i8: {small}");
    println!("  i16: {medium}");
    println!("  i32: {standard}");
    println!("  i64: {large}");
    println!("  i128: {huge}");

    // 无符号整数
    let unsigned: u32 = 4294967295;
    println!("  u32: {unsigned}");

    // 架构相关的整数
    let pointer_sized: isize = 64; // 依赖于架构
    println!("  isize: {pointer_sized}");

    // 浮点类型
    println!("\n浮点类型：");
    let f1 = 2.0; // f64（默认）
    let f2: f32 = 3.0; // f32

    println!("  f64: {f1}");
    println!("  f32: {f2}");

    // 科学记数法
    let scientific = 1e6; // 1,000,000
    println!("  科学记数法: {scientific}");

    // 布尔类型
    println!("\n布尔类型：");
    let t = true;
    let f: bool = false;

    println!("  true: {t}");
    println!("  false: {f}");

    // 字符类型
    println!("\n字符类型：");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("  ASCII 字符: {c}");
    println!("  Unicode 字符: {z}");
    println!("  Emoji: {heart_eyed_cat}");
}

fn compound_types_demo() {
    // 元组类型
    println!("元组类型：");
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 解构元组
    let (x, y, z) = tup;
    println!("  解构元组: x={x}, y={y}, z={z}");

    // 通过索引访问
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("  索引访问: {five_hundred}, {six_point_four}, {one}");

    // 单元类型（空元组）
    let unit = ();
    println!("  单元类型: {unit:?}");

    // 数组类型
    println!("\n数组类型：");
    let a = [1, 2, 3, 4, 5];
    println!("  数组: {a:?}");

    // 指定类型和长度
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("  指定类型数组: {b:?}");

    // 相同值初始化
    let c = [3; 5]; // [3, 3, 3, 3, 3]
    println!("  重复值数组: {c:?}");

    // 访问数组元素
    let first = a[0];
    let second = a[1];
    println!("  数组元素: first={first}, second={second}");

    // 多维数组
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("  二维数组: {matrix:?}");

    // 获取数组长度
    println!("  数组长度: {}", a.len());
}

fn type_inference_demo() {
    // 类型推断
    let guess: i32 = "42".parse().expect("不是一个数字!");
    println!("推断类型（需要类型注解）: {guess}");

    // 显式类型注解
    let guess: u32 = "42".parse().expect("不是一个数字!");
    println!("显式类型注解: {guess}");

    // 编译器可以推断的情况
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum(); // 从上下文推断
    println!("从上下文推断类型: {sum}");

    // 多种可能的类型
    let x = 5; // i32（默认）
    let y = 5.0; // f64（默认）
    println!("默认类型: x={x} (i32), y={y} (f64)");

    // 类型后缀
    let a = 5u32; // u32
    let b = 5.0f32; // f32
    println!("类型后缀: a={a} (u32), b={b} (f32)");
}

fn type_conversion_demo() {
    // as 关键字进行类型转换
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("使用 as 转换: {a} as u32 + {b} = {c}");

    // 可能丢失精度的转换
    let f = 3.14159f64;
    let i = f as i32;
    println!("浮点到整数转换: {f} as i32 = {i}");

    // 布尔值转换
    let t = true;
    let f = false;
    println!(
        "布尔值转换: true as i32 = {}, false as i32 = {}",
        t as i32, f as i32
    );

    // 字符转换
    let c = 'A';
    println!("字符转换: 'A' as u8 = {}", c as u8);

    // 使用 From/Into trait
    let s = "5";
    let i = i32::from_str_radix(s, 10).unwrap();
    println!("字符串解析: \"{s}\" -> {i}");

    // TryFrom 用于可能失败的转换
    use std::convert::TryFrom;
    let big_number: i64 = 1000;
    match i32::try_from(big_number) {
        Ok(small_number) => println!("成功转换: {big_number} -> {small_number}"),
        Err(e) => println!("转换失败: {e:?}"),
    }
}

fn numeric_operations_demo() {
    // 基本数学运算
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("数学运算:");
    println!("  加法: 5 + 10 = {sum}");
    println!("  减法: 95.5 - 4.3 = {difference}");
    println!("  乘法: 4 * 30 = {product}");
    println!("  除法: 56.7 / 32.2 = {quotient}");
    println!("  求余: 43 % 5 = {remainder}");

    // 整数溢出
    let max_u8 = u8::MAX;
    println!("\nu8 最大值: {max_u8}");

    // 在 debug 模式下会 panic，在 release 模式下会回绕
    // let overflow = max_u8 + 1; // 这会导致溢出

    // 安全的溢出处理
    let result = max_u8.wrapping_add(1);
    println!("回绕加法: {max_u8}.wrapping_add(1) = {result}");

    let result = max_u8.saturating_add(1);
    println!("饱和加法: {max_u8}.saturating_add(1) = {result}");

    let result = max_u8.checked_add(1);
    println!("检查加法: {max_u8}.checked_add(1) = {result:?}");

    // 位运算
    let a = 0b1100u8;
    let b = 0b1010u8;

    println!("\n位运算 (a={a:04b}, b={b:04b}):");
    println!("  AND: {a:04b} & {b:04b} = {:04b}", a & b);
    println!("  OR:  {a:04b} | {b:04b} = {:04b}", a | b);
    println!("  XOR: {a:04b} ^ {b:04b} = {:04b}", a ^ b);
    println!("  NOT: !{a:04b} = {:04b}", !a);
    println!("  左移: {a:04b} << 1 = {:04b}", a << 1);
    println!("  右移: {a:04b} >> 1 = {:04b}", a >> 1);
}

fn char_and_string_demo() {
    // 字符类型
    let c1 = 'a';
    let c2 = '中';
    let c3 = '🦀';

    println!("字符类型:");
    println!("  ASCII: {c1}");
    println!("  中文: {c2}");
    println!("  Emoji: {c3}");

    // 字符的大小
    println!("  字符大小: {} 字节", std::mem::size_of::<char>());

    // 字符串字面量
    let s1 = "Hello, world!";
    println!("\n字符串字面量: {s1}");

    // String 类型
    let mut s2 = String::from("Hello");
    s2.push_str(", world!");
    println!("String 类型: {s2}");

    // 原始字符串
    let raw_string = r#"这是一个原始字符串，包含 \ 和 " 等特殊字符"#;
    println!("原始字符串: {raw_string}");

    // 多行字符串
    let multiline = "这是第一行\n这是第二行\n这是第三行";
    println!("多行字符串:\n{multiline}");
}

// 辅助函数：类型信息
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

// 辅助函数：内存大小
fn print_type_info<T>(value: &T, name: &str) {
    println!(
        "{name}: 类型={}, 大小={} 字节",
        type_of(value),
        std::mem::size_of::<T>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_types() {
        let x: i32 = 42;
        assert_eq!(x, 42);

        let y: u8 = 255;
        assert_eq!(y, u8::MAX);
    }

    #[test]
    fn test_floating_point() {
        let x = 3.14159;
        assert!((x - 3.14159_f64).abs() < f64::EPSILON);

        let y: f32 = 2.71828;
        assert!((y - 2.71828f32).abs() < f32::EPSILON);
    }

    #[test]
    fn test_boolean() {
        let t = true;
        let f = false;

        assert_eq!(t, !f);
        assert_eq!(t && f, false);
        assert_eq!(t || f, true);
    }

    #[test]
    fn test_char() {
        let c = 'A';
        assert_eq!(c as u8, 65);

        let emoji = '😀';
        assert_eq!(emoji as u32, 0x1F600);
    }

    #[test]
    fn test_tuple() {
        let tup = (1, 2.5, 'a');
        let (x, y, z) = tup;

        assert_eq!(x, 1);
        assert_eq!(y, 2.5);
        assert_eq!(z, 'a');

        assert_eq!(tup.0, 1);
        assert_eq!(tup.1, 2.5);
        assert_eq!(tup.2, 'a');
    }

    #[test]
    fn test_array() {
        let arr = [1, 2, 3, 4, 5];

        assert_eq!(arr.len(), 5);
        assert_eq!(arr[0], 1);
        assert_eq!(arr[4], 5);

        let repeated = [3; 4];
        assert_eq!(repeated, [3, 3, 3, 3]);
    }

    #[test]
    fn test_type_conversion() {
        let x = 5u8;
        let y = x as u32;
        assert_eq!(y, 5u32);

        let f = 3.7f64;
        let i = f as i32;
        assert_eq!(i, 3);
    }

    #[test]
    fn test_overflow_handling() {
        let max = u8::MAX;

        assert_eq!(max.wrapping_add(1), 0);
        assert_eq!(max.saturating_add(1), 255);
        assert_eq!(max.checked_add(1), None);
    }

    #[test]
    fn test_bit_operations() {
        let a = 0b1100u8;
        let b = 0b1010u8;

        assert_eq!(a & b, 0b1000);
        assert_eq!(a | b, 0b1110);
        assert_eq!(a ^ b, 0b0110);
        assert_eq!(!a, 0b11110011);
    }
}
