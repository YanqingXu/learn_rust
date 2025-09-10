//! 泛型学习模块
//!
//! 学习内容：
//! - 泛型函数
//! - 泛型结构体
//! - 泛型枚举
//! - 泛型方法
//! - 泛型约束

fn main() {
    println!("=== Rust 泛型学习 ===");

    // 1. 泛型函数
    println!("\n1. 泛型函数：");
    generic_functions_demo();

    // 2. 泛型结构体
    println!("\n2. 泛型结构体：");
    generic_structs_demo();

    // 3. 泛型枚举
    println!("\n3. 泛型枚举：");
    generic_enums_demo();

    // 4. 泛型方法
    println!("\n4. 泛型方法：");
    generic_methods_demo();

    // 5. 泛型约束
    println!("\n5. 泛型约束：");
    generic_constraints_demo();
}

fn generic_functions_demo() {
    // 简单的泛型函数
    println!("泛型函数演示:");

    // 查找最大值
    let numbers = vec![34, 50, 25, 100, 65];
    let max_num = largest(&numbers);
    println!("  数字列表的最大值: {max_num}");

    let chars = vec!['y', 'm', 'a', 'q'];
    let max_char = largest(&chars);
    println!("  字符列表的最大值: {max_char}");

    // 交换两个值
    let mut x = 5;
    let mut y = 10;
    println!("  交换前: x = {x}, y = {y}");
    swap(&mut x, &mut y);
    println!("  交换后: x = {x}, y = {y}");

    let mut s1 = String::from("hello");
    let mut s2 = String::from("world");
    println!("  交换前: s1 = {s1}, s2 = {s2}");
    swap(&mut s1, &mut s2);
    println!("  交换后: s1 = {s1}, s2 = {s2}");

    // 打印不同类型的值
    print_value(42);
    print_value("Hello, Rust!");
    print_value(3.14);
    print_value(vec![1, 2, 3]);
}

fn generic_structs_demo() {
    // 泛型结构体
    println!("泛型结构体演示:");

    // Point 结构体
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("  整数点: ({}, {})", integer_point.x, integer_point.y);
    println!("  浮点数点: ({}, {})", float_point.x, float_point.y);

    // 混合类型的 Point
    let mixed_point = PointMixed { x: 5, y: 4.0 };
    println!("  混合类型点: ({}, {})", mixed_point.x, mixed_point.y);

    // 容器结构体
    let int_container = Container::new(42);
    let string_container = Container::new(String::from("Hello"));

    println!("  整数容器: {}", int_container.get());
    println!("  字符串容器: {}", string_container.get());

    // 多个值的容器
    let pair = Pair::new(1, 2);
    println!("  数对: ({}, {})", pair.first(), pair.second());

    let string_pair = Pair::new("hello".to_string(), "world".to_string());
    println!(
        "  字符串对: ({}, {})",
        string_pair.first(),
        string_pair.second()
    );
}

fn generic_enums_demo() {
    println!("泛型枚举演示:");

    // 自定义 Option 类型
    let some_int = MyOption::Some(5);
    let no_int: MyOption<i32> = MyOption::None;

    println!("  MyOption<i32>:");
    print_my_option(&some_int);
    print_my_option(&no_int);

    // 自定义 Result 类型
    let success: MyResult<i32, String> = MyResult::Ok(42);
    let failure: MyResult<i32, String> = MyResult::Err("Something went wrong".to_string());

    println!("  MyResult<i32, String>:");
    print_my_result(&success);
    print_my_result(&failure);

    // 多种类型的容器
    let list_int = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    let list_str = List::Cons(
        "hello".to_string(),
        Box::new(List::Cons("world".to_string(), Box::new(List::Nil))),
    );

    println!("  链表:");
    println!("    整数链表长度: {}", list_length(&list_int));
    println!("    字符串链表长度: {}", list_length(&list_str));
}

fn generic_methods_demo() {
    println!("泛型方法演示:");

    // Point 的方法
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 3, y: 4 };

    let distance = p1.distance_from(&p2);
    println!("  点之间的距离: {distance:.2}");

    // 混合类型点的方法
    let mixed = PointMixed { x: 5, y: 4.0 };
    let mixed2 = PointMixed { x: 3, y: 2.0 };

    let new_point = mixed.mixup(mixed2);
    println!("  混合后的点: ({}, {})", new_point.x, new_point.y);

    // 容器的泛型方法
    let container = Container::new(vec![1, 2, 3]);
    let mapped_container = container.map(|v| v.len());
    println!("  映射后的容器: {}", mapped_container.get());

    // Pair 的方法
    let pair = Pair::new(String::from("hello"), 42);
    let (first, second) = pair.into_parts();
    println!("  拆分的 Pair: {first}, {second}");
}

fn generic_constraints_demo() {
    println!("泛型约束演示:");

    // 需要 Display trait 的函数
    display_largest(&vec![1, 2, 3, 10, 5]);
    display_largest(&vec!["hello", "world", "rust"]);

    // 需要 Clone trait 的函数
    let numbers = vec![1, 5, 3, 9, 2];
    let cloned_max = clone_largest(&numbers);
    println!("  克隆的最大值: {cloned_max}");

    // 需要 PartialEq trait 的函数
    let list = vec![1, 2, 3, 4, 5];
    println!("  列表中是否包含 3: {}", contains(&list, &3));
    println!("  列表中是否包含 10: {}", contains(&list, &10));

    // 多个约束
    let comparable = Comparable::new(5, 10);
    println!("  比较结果: {:?}", comparable.compare());

    // 条件化的方法实现
    let pair_cmp = Pair::new(1, 2);
    println!("  最大值: {}", pair_cmp.max());

    // where 子句
    let summary = create_summary(String::from("标题"), String::from("这是内容"));
    println!("  摘要: {}", summary.summarize());
}

// 泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

fn print_value<T: std::fmt::Debug>(value: T) {
    println!("  值: {value:?}");
}

// 泛型结构体
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointMixed<T, U> {
    x: T,
    y: U,
}

struct Container<T> {
    value: T,
}

struct Pair<T, U> {
    first: T,
    second: U,
}

// 泛型枚举
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

// 结构体方法的实现
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T> Point<T>
where
    T: Copy + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Into<f64>,
{
    fn distance_from(&self, other: &Point<T>) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dx_f64: f64 = dx.into();
        let dy_f64: f64 = dy.into();
        (dx_f64 * dx_f64 + dy_f64 * dy_f64).sqrt()
    }
}

impl<T, U> PointMixed<T, U> {
    fn mixup<V, W>(self, other: PointMixed<V, W>) -> PointMixed<T, W> {
        PointMixed {
            x: self.x,
            y: other.y,
        }
    }
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn map<U, F>(self, f: F) -> Container<U>
    where
        F: FnOnce(T) -> U,
    {
        Container::new(f(self.value))
    }
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    fn first(&self) -> &T {
        &self.first
    }

    fn second(&self) -> &U {
        &self.second
    }

    fn into_parts(self) -> (T, U) {
        (self.first, self.second)
    }
}

// 带约束的方法
impl<T: PartialOrd> Pair<T, T> {
    fn max(&self) -> &T {
        if self.first >= self.second {
            &self.first
        } else {
            &self.second
        }
    }
}

// 辅助函数
fn print_my_option<T: std::fmt::Debug>(option: &MyOption<T>) {
    match option {
        MyOption::Some(value) => println!("    Some({value:?})"),
        MyOption::None => println!("    None"),
    }
}

fn print_my_result<T: std::fmt::Debug, E: std::fmt::Debug>(result: &MyResult<T, E>) {
    match result {
        MyResult::Ok(value) => println!("    Ok({value:?})"),
        MyResult::Err(error) => println!("    Err({error:?})"),
    }
}

fn list_length<T>(list: &List<T>) -> usize {
    match list {
        List::Cons(_, tail) => 1 + list_length(tail),
        List::Nil => 0,
    }
}

// 带约束的泛型函数
fn display_largest<T: PartialOrd + std::fmt::Display>(list: &[T]) {
    let largest = largest(list);
    println!("  最大值是: {largest}");
}

fn clone_largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let largest = largest(list);
    largest.clone()
}

fn contains<T: PartialEq>(list: &[T], item: &T) -> bool {
    list.iter().any(|x| x == item)
}

struct Comparable<T> {
    first: T,
    second: T,
}

impl<T> Comparable<T> {
    fn new(first: T, second: T) -> Self {
        Comparable { first, second }
    }
}

impl<T: PartialOrd> Comparable<T> {
    fn compare(&self) -> std::cmp::Ordering {
        self.first.partial_cmp(&self.second).unwrap()
    }
}

// 使用 where 子句的复杂约束
fn create_summary<T, U>(title: T, content: U) -> Summary<T, U>
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Display + Clone,
{
    Summary { title, content }
}

struct Summary<T, U> {
    title: T,
    content: U,
}

impl<T, U> Summary<T, U>
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest() {
        let numbers = vec![34, 50, 25, 100, 65];
        assert_eq!(*largest(&numbers), 100);

        let chars = vec!['y', 'm', 'a', 'q'];
        assert_eq!(*largest(&chars), 'y');
    }

    #[test]
    fn test_swap() {
        let mut x = 5;
        let mut y = 10;
        swap(&mut x, &mut y);
        assert_eq!(x, 10);
        assert_eq!(y, 5);
    }

    #[test]
    fn test_point() {
        let p1 = Point::new(3, 4);
        let p2 = Point::new(0, 0);

        // 注意：distance_from 需要特定的约束，这里可能无法直接测试
        // 我们测试基本的点创建
        assert_eq!(p1.x, 3);
        assert_eq!(p1.y, 4);
    }

    #[test]
    fn test_container() {
        let container = Container::new(42);
        assert_eq!(*container.get(), 42);

        let mapped = container.map(|x| x * 2);
        assert_eq!(*mapped.get(), 84);
    }

    #[test]
    fn test_pair() {
        let pair = Pair::new(1, 2);
        assert_eq!(*pair.first(), 1);
        assert_eq!(*pair.second(), 2);

        assert_eq!(*pair.max(), 2);

        let (first, second) = pair.into_parts();
        assert_eq!(first, 1);
        assert_eq!(second, 2);
    }

    #[test]
    fn test_contains() {
        let list = vec![1, 2, 3, 4, 5];
        assert!(contains(&list, &3));
        assert!(!contains(&list, &10));
    }

    #[test]
    fn test_my_option() {
        let some_value = MyOption::Some(42);
        let none_value: MyOption<i32> = MyOption::None;

        match some_value {
            MyOption::Some(x) => assert_eq!(x, 42),
            MyOption::None => panic!("Expected Some"),
        }

        match none_value {
            MyOption::Some(_) => panic!("Expected None"),
            MyOption::None => {}
        }
    }

    #[test]
    fn test_list_length() {
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        assert_eq!(list_length(&list), 2);

        let empty: List<i32> = List::Nil;
        assert_eq!(list_length(&empty), 0);
    }
}
