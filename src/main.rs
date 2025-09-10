//! Rust 学习项目主入口
//! 
//! 这是一个用于学习 Rust 编程语言的项目。
//! 包含了从基础语法到高级特性的各种示例代码。

fn main() {
    println!("🦀 欢迎来到 Rust 学习之旅！");
    println!();
    println!("📚 可用的学习模块：");
    println!("   基础语法: cargo run --bin variables, functions, control_flow, data_types");
    println!("   所有权系统: cargo run --bin ownership, references, slices");
    println!("   结构体枚举: cargo run --bin structs, enums, pattern_matching");
    println!("   错误处理: cargo run --bin error_handling, result_option");
    println!("   泛型特征: cargo run --bin generics, traits, lifetimes");
    println!("   集合类型: cargo run --bin collections, vectors, strings, hashmaps");
    println!("   函数式编程: cargo run --bin closures, iterators");
    println!("   并发编程: cargo run --bin threads, channels, shared_state");
    println!("   项目练习: cargo run --bin calculator, todo_app, file_search");
    println!();
    println!("💡 提示：使用 'cargo run --bin <模块名>' 来运行特定的学习模块");
    println!("📖 查看学习计划：cat rust_learning_plan.md");
    println!("📊 跟踪学习进度：编辑 learning_progress.md");
    println!();
    println!("🚀 开始你的 Rust 学习之旅吧！");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}