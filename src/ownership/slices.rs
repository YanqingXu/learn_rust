//! 切片学习模块
//! 
//! 学习内容：
//! - 字符串切片
//! - 数组切片
//! - 切片的语法和使用
//! - 切片作为函数参数

fn main() {
    println!("=== Rust 切片学习 ===");
    
    // 1. 字符串切片
    println!("\n1. 字符串切片：");
    string_slices_demo();
    
    // 2. 数组切片
    println!("\n2. 数组切片：");
    array_slices_demo();
    
    // 3. 切片的范围语法
    println!("\n3. 切片的范围语法：");
    slice_range_syntax_demo();
    
    // 4. 切片作为函数参数
    println!("\n4. 切片作为函数参数：");
    slice_parameters_demo();
    
    // 5. 可变切片
    println!("\n5. 可变切片：");
    mutable_slices_demo();
}

fn string_slices_demo() {
    let s = String::from("hello world");
    
    // 字符串切片
    let hello = &s[0..5];    // 或者 &s[..5]
    let world = &s[6..11];   // 或者 &s[6..]
    let whole = &s[..];      // 整个字符串
    
    println!("原字符串: {s}");
    println!("hello: {hello}");
    println!("world: {world}");
    println!("整个字符串: {whole}");
    
    // 使用 first_word 函数
    let word = first_word(&s);
    println!("第一个单词: {word}");
    
    // 字符串字面量就是切片
    let s_literal = "Hello, world!";
    println!("字符串字面量: {s_literal}");
    println!("字面量类型: &str");
    
    // 中文字符串切片（注意字节边界）
    let chinese = String::from("你好世界");
    let hello_cn = &chinese[0..6];   // "你好" 占6个字节
    let world_cn = &chinese[6..12];  // "世界" 占6个字节
    println!("中文字符串: {chinese}");
    println!("你好: {hello_cn}");
    println!("世界: {world_cn}");
}

fn array_slices_demo() {
    let a = [1, 2, 3, 4, 5];
    
    // 数组切片
    let slice = &a[1..4];    // [2, 3, 4]
    let first_three = &a[..3];  // [1, 2, 3]
    let last_two = &a[3..];     // [4, 5]
    let all = &a[..];           // [1, 2, 3, 4, 5]
    
    println!("原数组: {a:?}");
    println!("切片 [1..4]: {slice:?}");
    println!("前三个 [..3]: {first_three:?}");
    println!("后两个 [3..]: {last_two:?}");
    println!("全部 [..]: {all:?}");
    
    // 不同类型的数组
    let numbers: [i32; 6] = [10, 20, 30, 40, 50, 60];
    let middle = &numbers[2..4];
    println!("数字数组: {numbers:?}");
    println!("中间部分: {middle:?}");
    
    // 字符数组
    let chars = ['a', 'b', 'c', 'd', 'e'];
    let char_slice = &chars[1..4];
    println!("字符数组: {chars:?}");
    println!("字符切片: {char_slice:?}");
}

fn slice_range_syntax_demo() {
    let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    println!("原数据: {data:?}");
    
    // 不同的范围语法
    println!("不同的范围语法：");
    println!("  [2..5]: {:?}", &data[2..5]);     // [2, 3, 4]
    println!("  [..3]: {:?}", &data[..3]);       // [0, 1, 2]
    println!("  [7..]: {:?}", &data[7..]);       // [7, 8, 9]
    println!("  [..]: {:?}", &data[..]);         // 全部
    
    // 包含结束值的范围
    println!("  [2..=5]: {:?}", &data[2..=5]);   // [2, 3, 4, 5]
    
    // 使用变量作为索引
    let start = 3;
    let end = 7;
    let variable_slice = &data[start..end];
    println!("  [{start}..{end}]: {variable_slice:?}");
    
    // 字符串的范围
    let text = "Hello, Rust!";
    println!("\n字符串范围：");
    println!("  原文本: {text}");
    println!("  [0..5]: {}", &text[0..5]);      // "Hello"
    println!("  [7..]: {}", &text[7..]);        // "Rust!"
    println!("  [..5]: {}", &text[..5]);        // "Hello"
}

fn slice_parameters_demo() {
    let s = String::from("hello world programming");
    
    // 使用字符串切片作为参数的函数
    let word_count = count_words(&s);
    println!("单词数量: {word_count}");
    
    let contains_rust = contains_word(&s, "programming");
    println!("包含 'programming': {contains_rust}");
    
    // 数组切片作为参数
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = sum_slice(&numbers);
    println!("数组总和: {sum}");
    
    let sum_part = sum_slice(&numbers[2..7]);
    println!("部分总和 [2..7]: {sum_part}");
    
    // 查找最大值
    let max = find_max(&numbers);
    println!("最大值: {max:?}");
    
    // 空切片
    let empty: &[i32] = &[];
    let empty_max = find_max(empty);
    println!("空切片的最大值: {empty_max:?}");
}

fn mutable_slices_demo() {
    let mut arr = [1, 2, 3, 4, 5];
    println!("原数组: {arr:?}");
    
    // 可变切片
    let slice = &mut arr[1..4];
    slice[0] = 10;  // 修改原数组的第二个元素
    slice[1] = 20;  // 修改原数组的第三个元素
    
    println!("修改后的数组: {arr:?}");
    
    // 使用可变切片的函数
    let mut data = [5, 2, 8, 1, 9, 3];
    println!("排序前: {data:?}");
    
    sort_slice(&mut data);
    println!("排序后: {data:?}");
    
    // 部分排序
    let mut mixed = [9, 2, 7, 4, 1, 8, 5];
    println!("部分排序前: {mixed:?}");
    sort_slice(&mut mixed[1..5]);  // 只排序中间部分
    println!("部分排序后: {mixed:?}");
    
    // 交换元素
    let mut values = [10, 20, 30, 40, 50];
    println!("交换前: {values:?}");
    swap_elements(&mut values, 1, 3);
    println!("交换后: {values:?}");
}

// 查找第一个单词
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 计算单词数量
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

// 检查是否包含特定单词
fn contains_word(s: &str, word: &str) -> bool {
    s.split_whitespace().any(|w| w == word)
}

// 计算切片的总和
fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

// 查找切片中的最大值
fn find_max(slice: &[i32]) -> Option<i32> {
    if slice.is_empty() {
        None
    } else {
        Some(*slice.iter().max().unwrap())
    }
}

// 排序可变切片
fn sort_slice(slice: &mut [i32]) {
    slice.sort();
}

// 交换两个元素
fn swap_elements(slice: &mut [i32], i: usize, j: usize) {
    if i < slice.len() && j < slice.len() {
        slice.swap(i, j);
    }
}

// 获取字符串的安全子串
fn safe_substring(s: &str, start: usize, len: usize) -> Option<&str> {
    let end = start + len;
    if end <= s.len() {
        Some(&s[start..end])
    } else {
        None
    }
}

// 反转切片
fn reverse_slice(slice: &mut [i32]) {
    slice.reverse();
}

// 查找子切片
fn find_subslice(haystack: &[i32], needle: &[i32]) -> Option<usize> {
    haystack.windows(needle.len())
        .position(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("hello"), "hello");
        assert_eq!(first_word(""), "");
    }
    
    #[test]
    fn test_count_words() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words("one"), 1);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("  multiple   spaces  "), 2);
    }
    
    #[test]
    fn test_contains_word() {
        assert!(contains_word("hello world", "hello"));
        assert!(contains_word("hello world", "world"));
        assert!(!contains_word("hello world", "rust"));
    }
    
    #[test]
    fn test_sum_slice() {
        assert_eq!(sum_slice(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_slice(&[]), 0);
        assert_eq!(sum_slice(&[10]), 10);
    }
    
    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[1, 5, 3, 9, 2]), Some(9));
        assert_eq!(find_max(&[]), None);
        assert_eq!(find_max(&[42]), Some(42));
    }
    
    #[test]
    fn test_sort_slice() {
        let mut arr = [3, 1, 4, 1, 5];
        sort_slice(&mut arr);
        assert_eq!(arr, [1, 1, 3, 4, 5]);
    }
    
    #[test]
    fn test_swap_elements() {
        let mut arr = [1, 2, 3, 4, 5];
        swap_elements(&mut arr, 0, 4);
        assert_eq!(arr, [5, 2, 3, 4, 1]);
    }
    
    #[test]
    fn test_safe_substring() {
        let s = "hello";
        assert_eq!(safe_substring(s, 0, 2), Some("he"));
        assert_eq!(safe_substring(s, 1, 3), Some("ell"));
        assert_eq!(safe_substring(s, 0, 10), None);
        assert_eq!(safe_substring(s, 10, 1), None);
    }
    
    #[test]
    fn test_reverse_slice() {
        let mut arr = [1, 2, 3, 4, 5];
        reverse_slice(&mut arr);
        assert_eq!(arr, [5, 4, 3, 2, 1]);
    }
    
    #[test]
    fn test_find_subslice() {
        let haystack = [1, 2, 3, 4, 5, 6];
        assert_eq!(find_subslice(&haystack, &[3, 4]), Some(2));
        assert_eq!(find_subslice(&haystack, &[5, 6]), Some(4));
        assert_eq!(find_subslice(&haystack, &[7, 8]), None);
    }
    
    #[test]
    fn test_string_slices() {
        let s = String::from("hello world");
        assert_eq!(&s[0..5], "hello");
        assert_eq!(&s[6..], "world");
        assert_eq!(&s[..], "hello world");
    }
    
    #[test]
    fn test_array_slices() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(&arr[1..4], &[2, 3, 4]);
        assert_eq!(&arr[..3], &[1, 2, 3]);
        assert_eq!(&arr[3..], &[4, 5]);
    }
}
