//! 引用和借用学习模块
//! 
//! 学习内容：
//! - 引用的概念
//! - 可变引用和不可变引用
//! - 借用规则
//! - 悬垂引用的避免

fn main() {
    println!("=== Rust 引用和借用学习 ===");
    
    // 1. 基本引用
    println!("\n1. 基本引用：");
    basic_references_demo();
    
    // 2. 可变引用
    println!("\n2. 可变引用：");
    mutable_references_demo();
    
    // 3. 借用规则
    println!("\n3. 借用规则：");
    borrowing_rules_demo();
    
    // 4. 引用的作用域
    println!("\n4. 引用的作用域：");
    reference_scope_demo();
    
    // 5. 函数参数中的引用
    println!("\n5. 函数参数中的引用：");
    function_parameters_demo();
}

fn basic_references_demo() {
    let s1 = String::from("hello");
    
    // 创建引用，不会获取所有权
    let len = calculate_length(&s1);
    
    println!("字符串 '{}' 的长度是 {}。", s1, len);
    // s1 在这里仍然有效，因为我们没有移动它
    
    // 多个不可变引用
    let r1 = &s1;
    let r2 = &s1;
    println!("r1: {}, r2: {}", r1, r2);
    
    // 引用的引用
    let s = String::from("world");
    let r = &s;        // r 是 String 的引用
    let rr = &r;       // rr 是引用的引用
    println!("s: {}, r: {}, rr: {}", s, r, rr);
}

fn mutable_references_demo() {
    let mut s = String::from("hello");
    
    // 创建可变引用
    change(&mut s);
    
    println!("修改后的字符串: {}", s);
    
    // 一次只能有一个可变引用
    let r1 = &mut s;
    r1.push_str(", world");
    println!("通过可变引用修改: {}", r1);
    
    // r1 在这里不再使用，所以可以创建新的引用
    let r2 = &s;
    println!("创建新的不可变引用: {}", r2);
}

fn borrowing_rules_demo() {
    println!("借用规则演示：");
    
    let mut s = String::from("hello");
    
    // 规则1：任意时刻，要么有一个可变引用，要么有任意数量的不可变引用
    {
        let r1 = &s;     // 没问题
        let r2 = &s;     // 没问题
        println!("r1: {}, r2: {}", r1, r2);
        // r1 和 r2 在这里不再使用
    }
    
    {
        let r3 = &mut s; // 没问题
        r3.push_str(", world");
        println!("r3: {}", r3);
        // r3 在这里不再使用
    }
    
    // 规则2：引用必须总是有效的
    println!("最终的字符串: {}", s);
    
    // 演示借用检查器的工作
    demonstrate_borrow_checker();
}

fn demonstrate_borrow_checker() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    // 同时拥有不可变引用
    let first = &data[0];
    let second = &data[1];
    
    println!("第一个元素: {}, 第二个元素: {}", first, second);
    
    // 在不可变引用使用完毕后，可以创建可变引用
    data.push(6);
    println!("添加元素后: {:?}", data);
    
    // 演示作用域规则
    let r1 = &data;
    let r2 = &data;
    println!("r1: {:?}, r2: {:?}", r1, r2);
    // r1 和 r2 的生命周期在这里结束
    
    let r3 = &mut data;
    r3.push(7);
    println!("r3: {:?}", r3);
}

fn reference_scope_demo() {
    let mut s = String::from("hello");
    
    let r1 = &s;          // 没问题
    let r2 = &s;          // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    
    let r3 = &mut s;      // 没问题
    println!("{}", r3);
    
    // 非词法作用域生命周期 (NLL) 的示例
    let data = vec![1, 2, 3];
    let first = &data[0];           // 开始借用
    println!("第一个元素: {}", first); // 最后使用借用
    // 借用在这里结束，即使作用域还没结束
    
    // 现在可以获取可变引用了（在新版本 Rust 中）
    // let mut data = data; // 如果需要修改
}

fn function_parameters_demo() {
    let s = String::from("hello world");
    
    // 传递不可变引用
    let word = first_word(&s);
    println!("第一个单词: {}", word);
    
    let mut s2 = String::from("hello world");
    
    // 传递可变引用
    make_uppercase(&mut s2);
    println!("转换为大写: {}", s2);
    
    // 返回引用
    let longest = longest_string("hello", "world");
    println!("最长的字符串: {}", longest);
    
    // 多个引用参数
    let result = compare_strings(&s, &s2);
    println!("字符串比较结果: {:?}", result);
}

// 计算字符串长度，不获取所有权
fn calculate_length(s: &String) -> usize {
    s.len()
} // s 在这里超出作用域，但因为它不拥有引用值，所以什么也不会发生

// 修改通过引用传入的字符串
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 返回第一个单词的字符串切片
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 将字符串转换为大写
fn make_uppercase(s: &mut String) {
    *s = s.to_uppercase();
}

// 返回最长的字符串（需要生命周期参数）
fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 比较两个字符串
fn compare_strings(s1: &String, s2: &String) -> std::cmp::Ordering {
    s1.cmp(s2)
}

// 演示引用作为结构体字段（需要生命周期）
struct StringHolder<'a> {
    content: &'a str,
}

impl<'a> StringHolder<'a> {
    fn new(s: &'a str) -> StringHolder<'a> {
        StringHolder { content: s }
    }
    
    fn get_content(&self) -> &str {
        self.content
    }
    
    fn get_length(&self) -> usize {
        self.content.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_references() {
        let s = String::from("hello");
        let len = calculate_length(&s);
        
        assert_eq!(len, 5);
        assert_eq!(s, "hello"); // s 仍然有效
    }
    
    #[test]
    fn test_mutable_references() {
        let mut s = String::from("hello");
        change(&mut s);
        
        assert_eq!(s, "hello, world");
    }
    
    #[test]
    fn test_first_word() {
        let s = String::from("hello world");
        let word = first_word(&s);
        
        assert_eq!(word, "hello");
        
        let s2 = String::from("hello");
        let word2 = first_word(&s2);
        assert_eq!(word2, "hello");
    }
    
    #[test]
    fn test_longest_string() {
        assert_eq!(longest_string("hello", "world"), "world");
        assert_eq!(longest_string("rust", "go"), "rust");
    }
    
    #[test]
    fn test_string_holder() {
        let s = "hello, world";
        let holder = StringHolder::new(s);
        
        assert_eq!(holder.get_content(), "hello, world");
        assert_eq!(holder.get_length(), 12);
    }
    
    #[test]
    fn test_multiple_immutable_references() {
        let s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        
        assert_eq!(r1, "hello");
        assert_eq!(r2, "hello");
        assert_eq!(r1, r2);
    }
    
    #[test]
    fn test_reference_scope() {
        let mut s = String::from("hello");
        
        // 多个不可变引用
        let r1 = &s;
        let r2 = &s;
        assert_eq!(r1, r2);
        // r1 和 r2 不再使用
        
        // 现在可以创建可变引用
        let r3 = &mut s;
        r3.push_str(" world");
        assert_eq!(r3, "hello world");
    }
}
