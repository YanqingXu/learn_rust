//! 变量和可变性学习模块
//!
//! 学习内容：
//! - 变量声明和绑定
//! - 可变性 (mut)
//! - 常量 (const)
//! - 变量遮蔽 (shadowing)
//! - 作用域

fn main() {
    println!("=== Rust 变量和可变性学习 ===");

    // 1. 不可变变量（默认）
    println!("\n1. 不可变变量：");
    let x = 5;
    println!("x = {}", x);
    // x = 6; // 这行会编译错误，因为 x 是不可变的

    // 2. 可变变量
    println!("\n2. 可变变量：");
    let mut y = 5;
    println!("y 的初始值: {}", y);
    y = 6;
    println!("y 的新值: {}", y);

    // 3. 常量
    println!("\n3. 常量：");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("三小时的秒数: {}", THREE_HOURS_IN_SECONDS);

    // 4. 变量遮蔽
    println!("\n4. 变量遮蔽：");
    let z = 5;
    println!("z 的第一个值: {}", z);

    let z = z + 1; // 遮蔽前一个 z
    println!("z 的第二个值: {}", z);

    {
        let z = z * 2; // 在内部作用域中遮蔽
        println!("内部作用域中 z 的值: {}", z);
    }

    println!("外部作用域中 z 的值: {}", z);

    // 5. 遮蔽允许改变类型
    println!("\n5. 遮蔽改变类型：");
    let spaces = "   ";
    println!("spaces 是字符串: '{}'", spaces);

    let spaces = spaces.len();
    println!("spaces 现在是数字: {}", spaces);

    // 6. 变量作用域演示
    println!("\n6. 变量作用域：");
    demonstrate_scope();

    // 7. 解构赋值
    println!("\n7. 解构赋值：");
    let (a, b) = (1, 2);
    println!("a = {}, b = {}", a, b);

    let (mut x, y) = (1, 2);
    x += y;
    println!("x = {}, y = {}", x, y);
}

fn demonstrate_scope() {
    let outer_var = "我在外部作用域";
    println!("外部变量: {}", outer_var);

    {
        let inner_var = "我在内部作用域";
        println!("内部变量: {}", inner_var);
        println!("内部作用域也能访问外部变量: {}", outer_var);

        // 内部作用域的变量遮蔽
        let outer_var = "我遮蔽了外部变量";
        println!("遮蔽后的外部变量: {}", outer_var);
    }

    println!("回到外部作用域: {}", outer_var);
    // println!("{}", inner_var); // 这行会编译错误，inner_var 已超出作用域
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_variables() {
        let x = 5;
        assert_eq!(x, 5);

        let mut y = 10;
        y = 15;
        assert_eq!(y, 15);
    }

    #[test]
    fn test_shadowing() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        assert_eq!(x, 12);
    }
}
