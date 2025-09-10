//! Rust 学习项目库
//! 
//! 这个库包含了学习 Rust 编程语言的各种模块和示例。
//! 每个模块都专注于 Rust 的特定概念或特性。

// 基础语法模块
pub mod basics {
    //! 基础语法学习模块
    //! 
    //! 包含变量、函数、控制流、数据类型等基础概念
    
    // 注意：这些模块在各自的二进制文件中定义
    // 这里只是为了文档和组织结构
}

// 所有权系统模块
pub mod ownership {
    //! 所有权系统学习模块
    //! 
    //! Rust 最重要的特性：所有权、借用、生命周期
}

// 结构体和枚举模块
pub mod structs_enums {
    //! 结构体和枚举学习模块
    //! 
    //! 自定义数据类型、模式匹配、方法定义
}

// 错误处理模块
pub mod error_handling {
    //! 错误处理学习模块
    //! 
    //! Result、Option、panic!、错误传播
}

// 泛型和特征模块
pub mod generics_traits {
    //! 泛型和特征学习模块
    //! 
    //! 泛型编程、特征定义和实现、生命周期参数
}

// 集合类型模块
pub mod collections {
    //! 集合类型学习模块
    //! 
    //! Vector、String、HashMap 等常用集合
}

// 函数式编程模块
pub mod functional {
    //! 函数式编程学习模块
    //! 
    //! 闭包、迭代器、函数式编程模式
}

// 并发编程模块
pub mod concurrency {
    //! 并发编程学习模块
    //! 
    //! 线程、消息传递、共享状态、同步原语
}

// 项目练习模块
pub mod projects {
    //! 实践项目模块
    //! 
    //! 综合性项目，用于巩固所学知识
}

/// 学习进度跟踪
pub mod progress {
    //! 学习进度跟踪工具
    
    use std::collections::HashMap;
    
    /// 学习主题
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Topic {
        Variables,
        Functions,
        ControlFlow,
        DataTypes,
        Ownership,
        References,
        Slices,
        Structs,
        Enums,
        PatternMatching,
        ErrorHandling,
        ResultOption,
        Generics,
        Traits,
        Lifetimes,
        Collections,
        Vectors,
        Strings,
        HashMaps,
        Closures,
        Iterators,
        Threads,
        Channels,
        SharedState,
        Calculator,
        TodoApp,
        FileSearch,
    }
    
    /// 学习状态
    #[derive(Debug, Clone, PartialEq)]
    pub enum Status {
        NotStarted,
        InProgress,
        Completed,
        NeedsReview,
    }
    
    /// 学习进度跟踪器
    #[derive(Debug)]
    pub struct ProgressTracker {
        progress: HashMap<Topic, Status>,
    }
    
    impl ProgressTracker {
        /// 创建新的进度跟踪器
        pub fn new() -> Self {
            let mut tracker = Self {
                progress: HashMap::new(),
            };
            
            // 初始化所有主题为未开始状态
            let topics = [
                Topic::Variables, Topic::Functions, Topic::ControlFlow, Topic::DataTypes,
                Topic::Ownership, Topic::References, Topic::Slices,
                Topic::Structs, Topic::Enums, Topic::PatternMatching,
                Topic::ErrorHandling, Topic::ResultOption,
                Topic::Generics, Topic::Traits, Topic::Lifetimes,
                Topic::Collections, Topic::Vectors, Topic::Strings, Topic::HashMaps,
                Topic::Closures, Topic::Iterators,
                Topic::Threads, Topic::Channels, Topic::SharedState,
                Topic::Calculator, Topic::TodoApp, Topic::FileSearch,
            ];
            
            for topic in topics.iter() {
                tracker.progress.insert(topic.clone(), Status::NotStarted);
            }
            
            tracker
        }
        
        /// 更新主题状态
        pub fn update_status(&mut self, topic: Topic, status: Status) {
            self.progress.insert(topic, status);
        }
        
        /// 获取主题状态
        pub fn get_status(&self, topic: &Topic) -> Option<&Status> {
            self.progress.get(topic)
        }
        
        /// 获取完成的主题数量
        pub fn completed_count(&self) -> usize {
            self.progress.values()
                .filter(|&status| *status == Status::Completed)
                .count()
        }
        
        /// 获取总主题数量
        pub fn total_count(&self) -> usize {
            self.progress.len()
        }
        
        /// 计算完成百分比
        pub fn completion_percentage(&self) -> f64 {
            if self.total_count() == 0 {
                0.0
            } else {
                (self.completed_count() as f64 / self.total_count() as f64) * 100.0
            }
        }
        
        /// 显示进度报告
        pub fn show_progress(&self) {
            println!("🦀 Rust 学习进度报告");
            println!("====================");
            println!("完成进度: {:.1}% ({}/{})", 
                     self.completion_percentage(), 
                     self.completed_count(), 
                     self.total_count());
            println!();
            
            // 按阶段分组显示
            self.show_stage_progress("基础语法", &[
                Topic::Variables, Topic::Functions, Topic::ControlFlow, Topic::DataTypes
            ]);
            
            self.show_stage_progress("所有权系统", &[
                Topic::Ownership, Topic::References, Topic::Slices
            ]);
            
            self.show_stage_progress("结构体和枚举", &[
                Topic::Structs, Topic::Enums, Topic::PatternMatching
            ]);
            
            self.show_stage_progress("错误处理", &[
                Topic::ErrorHandling, Topic::ResultOption
            ]);
            
            self.show_stage_progress("泛型和特征", &[
                Topic::Generics, Topic::Traits, Topic::Lifetimes
            ]);
            
            self.show_stage_progress("集合类型", &[
                Topic::Collections, Topic::Vectors, Topic::Strings, Topic::HashMaps
            ]);
            
            self.show_stage_progress("函数式编程", &[
                Topic::Closures, Topic::Iterators
            ]);
            
            self.show_stage_progress("并发编程", &[
                Topic::Threads, Topic::Channels, Topic::SharedState
            ]);
            
            self.show_stage_progress("实践项目", &[
                Topic::Calculator, Topic::TodoApp, Topic::FileSearch
            ]);
        }
        
        fn show_stage_progress(&self, stage_name: &str, topics: &[Topic]) {
            println!("📚 {}", stage_name);
            for topic in topics {
                let status = self.get_status(topic).unwrap_or(&Status::NotStarted);
                let icon = match status {
                    Status::NotStarted => "⭕",
                    Status::InProgress => "🔄",
                    Status::Completed => "✅",
                    Status::NeedsReview => "🔍",
                };
                println!("   {} {:?}", icon, topic);
            }
            println!();
        }
    }
    
    impl Default for ProgressTracker {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// 学习资源和工具
pub mod utils {
    //! 学习辅助工具
    
    /// 代码运行计时器
    pub struct Timer {
        start: std::time::Instant,
    }
    
    impl Timer {
        pub fn new() -> Self {
            Self {
                start: std::time::Instant::now(),
            }
        }
        
        pub fn elapsed(&self) -> std::time::Duration {
            self.start.elapsed()
        }
        
        pub fn elapsed_ms(&self) -> u128 {
            self.elapsed().as_millis()
        }
    }
    
    impl Default for Timer {
        fn default() -> Self {
            Self::new()
        }
    }
    
    /// 打印分隔线
    pub fn print_separator(title: &str) {
        let line = "=".repeat(50);
        println!("{}", line);
        println!("🦀 {}", title);
        println!("{}", line);
    }
    
    /// 打印子标题
    pub fn print_subtitle(subtitle: &str) {
        println!("\n📖 {}", subtitle);
        println!("{}", "-".repeat(30));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_progress_tracker() {
        let mut tracker = progress::ProgressTracker::new();
        
        // 测试初始状态
        assert_eq!(tracker.completed_count(), 0);
        assert_eq!(tracker.completion_percentage(), 0.0);
        
        // 更新一些状态
        tracker.update_status(progress::Topic::Variables, progress::Status::Completed);
        tracker.update_status(progress::Topic::Functions, progress::Status::InProgress);
        
        assert_eq!(tracker.completed_count(), 1);
        assert!(tracker.completion_percentage() > 0.0);
        
        // 测试状态获取
        assert_eq!(
            tracker.get_status(&progress::Topic::Variables),
            Some(&progress::Status::Completed)
        );
    }
    
    #[test]
    fn test_timer() {
        let timer = utils::Timer::new();
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(timer.elapsed_ms() >= 10);
    }
}