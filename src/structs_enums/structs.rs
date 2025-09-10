//! 结构体学习模块
//! 
//! 学习内容：
//! - 结构体定义和实例化
//! - 方法和关联函数
//! - 结构体的所有权
//! - 元组结构体和单元结构体

fn main() {
    println!("=== Rust 结构体学习 ===");
    
    // 1. 基本结构体
    println!("\n1. 基本结构体：");
    basic_struct_demo();
    
    // 2. 方法语法
    println!("\n2. 方法语法：");
    method_syntax_demo();
    
    // 3. 关联函数
    println!("\n3. 关联函数：");
    associated_functions_demo();
    
    // 4. 元组结构体
    println!("\n4. 元组结构体：");
    tuple_struct_demo();
    
    // 5. 单元结构体
    println!("\n5. 单元结构体：");
    unit_struct_demo();
    
    // 6. 结构体的所有权
    println!("\n6. 结构体的所有权：");
    struct_ownership_demo();
}

// 基本结构体定义
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 矩形结构体
#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 带生命周期的结构体
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u32,
}

fn basic_struct_demo() {
    // 创建结构体实例
    let user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    println!("用户信息: {user1:?}");
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
    
    // 创建可变实例
    let mut user2 = User {
        active: true,
        username: String::from("anotheruser"),
        email: String::from("another@example.com"),
        sign_in_count: 1,
    };
    
    // 修改字段
    user2.email = String::from("newemail@example.com");
    println!("修改后的邮箱: {}", user2.email);
    
    // 使用函数创建实例
    let user3 = build_user(
        String::from("user@test.com"), 
        String::from("testuser")
    );
    println!("通过函数创建的用户: {user3:?}");
    
    // 结构体更新语法
    let user4 = User {
        email: String::from("updated@example.com"),
        ..user3  // 使用 user3 的其他字段值
    };
    println!("使用更新语法的用户: {user4:?}");
    // 注意：user3 的 username 所有权被移动到 user4
}

fn method_syntax_demo() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("矩形: {rect1:?}");
    println!("矩形面积: {}", rect1.area());
    println!("矩形周长: {}", rect1.perimeter());
    println!("是否为正方形: {}", rect1.is_square());
    
    // 创建另一个矩形
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    println!("rect1 能容纳 rect2 吗？{}", rect1.can_hold(&rect2));
    
    // 链式方法调用
    let rect3 = Rectangle { width: 20, height: 20 };
    let scaled = rect3.scale(2);
    println!("缩放后的矩形: {scaled:?}");
}

fn associated_functions_demo() {
    // 使用关联函数创建实例
    let square = Rectangle::square(25);
    println!("正方形: {square:?}");
    println!("正方形面积: {}", square.area());
    
    // 创建不同大小的矩形
    let rect = Rectangle::new(15, 25);
    println!("新矩形: {rect:?}");
    
    // 创建用户
    let admin = User::new_admin("admin", "admin@example.com");
    println!("管理员用户: {admin:?}");
}

fn tuple_struct_demo() {
    // 定义元组结构体
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("黑色: {black:?}");
    println!("原点: {origin:?}");
    
    // 访问元组结构体的字段
    println!("黑色的红色分量: {}", black.0);
    println!("原点的 x 坐标: {}", origin.0);
    
    // 元组结构体的解构
    let Color(r, g, b) = black;
    println!("RGB 值: r={r}, g={g}, b={b}");
}

fn unit_struct_demo() {
    // 单元结构体（没有字段）
    #[derive(Debug)]
    struct AlwaysEqual;
    
    let subject = AlwaysEqual;
    println!("单元结构体: {subject:?}");
    
    // 单元结构体常用于实现特征
    println!("单元结构体大小: {} 字节", std::mem::size_of::<AlwaysEqual>());
}

fn struct_ownership_demo() {
    // 结构体拥有其数据
    let user = User {
        active: true,
        username: String::from("owner"),
        email: String::from("owner@example.com"),
        sign_in_count: 1,
    };
    
    // 移动结构体
    let moved_user = user;
    println!("移动后的用户: {moved_user:?}");
    // println!("{:?}", user); // 这会编译错误，user 已被移动
    
    // 使用引用的结构体
    let name = "borrowed";
    let person = Person { name, age: 30 };
    println!("借用字符串的结构体: {person:?}");
    
    // 克隆结构体
    let original = Rectangle { width: 10, height: 20 };
    let cloned = original.clone();
    println!("原始矩形: {original:?}");
    println!("克隆矩形: {cloned:?}");
}

// 构建用户的函数
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,  // 字段初始化简写
        email,     // 字段初始化简写
        sign_in_count: 1,
    }
}

// 为 Rectangle 实现方法
impl Rectangle {
    // 计算面积
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 计算周长
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    // 判断是否为正方形
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    // 判断是否能容纳另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 缩放矩形
    fn scale(&self, factor: u32) -> Rectangle {
        Rectangle {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
    
    // 修改矩形大小
    fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }
}

// 为 Rectangle 实现关联函数
impl Rectangle {
    // 创建正方形
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // 创建新矩形
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // 创建默认矩形
    fn default() -> Rectangle {
        Rectangle {
            width: 1,
            height: 1,
        }
    }
}

// 为 User 实现方法
impl User {
    // 创建新用户
    fn new(username: String, email: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
    
    // 创建管理员用户
    fn new_admin(username: &str, email: &str) -> User {
        User {
            active: true,
            username: username.to_string(),
            email: email.to_string(),
            sign_in_count: 0,
        }
    }
    
    // 用户登录
    fn sign_in(&mut self) {
        self.sign_in_count += 1;
    }
    
    // 停用用户
    fn deactivate(&mut self) {
        self.active = false;
    }
    
    // 检查是否为新用户
    fn is_new_user(&self) -> bool {
        self.sign_in_count <= 1
    }
}

// 多个 impl 块
impl Rectangle {
    // 计算对角线长度
    fn diagonal(&self) -> f64 {
        ((self.width.pow(2) + self.height.pow(2)) as f64).sqrt()
    }
    
    // 与另一个矩形比较面积
    fn compare_area(&self, other: &Rectangle) -> std::cmp::Ordering {
        self.area().cmp(&other.area())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new(
            String::from("testuser"), 
            String::from("test@example.com")
        );
        
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert!(user.active);
        assert_eq!(user.sign_in_count, 1);
    }
    
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.area(), 200);
    }
    
    #[test]
    fn test_rectangle_perimeter() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.perimeter(), 60);
    }
    
    #[test]
    fn test_square() {
        let square = Rectangle::square(5);
        assert_eq!(square.width, 5);
        assert_eq!(square.height, 5);
        assert!(square.is_square());
    }
    
    #[test]
    fn test_can_hold() {
        let larger = Rectangle::new(10, 20);
        let smaller = Rectangle::new(5, 10);
        
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }
    
    #[test]
    fn test_user_sign_in() {
        let mut user = User::new_admin("admin", "admin@test.com");
        assert_eq!(user.sign_in_count, 0);
        
        user.sign_in();
        assert_eq!(user.sign_in_count, 1);
        
        user.sign_in();
        assert_eq!(user.sign_in_count, 2);
    }
    
    #[test]
    fn test_user_deactivate() {
        let mut user = User::new(
            String::from("active"), 
            String::from("active@test.com")
        );
        assert!(user.active);
        
        user.deactivate();
        assert!(!user.active);
    }
    
    #[test]
    fn test_rectangle_scale() {
        let rect = Rectangle::new(3, 4);
        let scaled = rect.scale(2);
        
        assert_eq!(scaled.width, 6);
        assert_eq!(scaled.height, 8);
    }
    
    #[test]
    fn test_rectangle_diagonal() {
        let rect = Rectangle::new(3, 4);
        let diagonal = rect.diagonal();
        
        // 3-4-5 直角三角形
        assert!((diagonal - 5.0).abs() < f64::EPSILON);
    }
}
