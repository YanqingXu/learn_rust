use rayon::prelude::*;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// 基本并行迭代器示例
fn basic_parallel_operations() {
    println!("=== 基本并行操作 ===");
    
    let numbers: Vec<i32> = (1..=1000).collect();
    
    // 并行映射
    let start = Instant::now();
    let squares: Vec<i32> = numbers.par_iter().map(|&x| x * x).collect();
    println!("并行计算平方耗时: {:?}", start.elapsed());
    println!("前10个平方: {:?}", &squares[0..10]);
    
    // 并行过滤
    let start = Instant::now();
    let evens: Vec<i32> = numbers.par_iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("并行过滤偶数耗时: {:?}", start.elapsed());
    println!("偶数个数: {}", evens.len());
    
    // 并行归约
    let start = Instant::now();
    let sum: i32 = numbers.par_iter().sum();
    println!("并行求和耗时: {:?}", start.elapsed());
    println!("总和: {}", sum);
    
    // 并行查找
    let start = Instant::now();
    let found = numbers.par_iter().find_any(|&&x| x == 500);
    println!("并行查找耗时: {:?}", start.elapsed());
    println!("找到的数字: {:?}", found);
}

// 并行排序示例
fn parallel_sorting() {
    println!("\n=== 并行排序 ===");
    
    let mut data: Vec<i32> = (0..100000).rev().collect(); // 逆序数据
    
    let start = Instant::now();
    data.par_sort();
    println!("并行排序耗时: {:?}", start.elapsed());
    println!("排序后前10个元素: {:?}", &data[0..10]);
    println!("排序后后10个元素: {:?}", &data[data.len()-10..]);
}

// 自定义并行算法：并行快速排序
fn parallel_quicksort<T: Send + Sync + Ord + Clone>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }
    
    if data.len() < 1000 {
        // 对于小数组，使用串行排序
        data.sort();
        return;
    }
    
    let pivot_index = partition(data);
    let (left, right) = data.split_at_mut(pivot_index);
    
    rayon::join(
        || parallel_quicksort(left),
        || parallel_quicksort(&mut right[1..]),
    );
}

fn partition<T: Ord>(data: &mut [T]) -> usize {
    let len = data.len();
    let pivot_index = len / 2;
    data.swap(pivot_index, len - 1);
    
    let mut i = 0;
    for j in 0..len - 1 {
        if data[j] <= data[len - 1] {
            data.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, len - 1);
    i
}

// 并行数据处理示例
fn parallel_data_processing() {
    println!("\n=== 并行数据处理 ===");
    
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
        salary: f64,
    }
    
    let people: Vec<Person> = (0..10000)
        .map(|i| Person {
            name: format!("Person_{}", i),
            age: 20 + (i % 50) as u32,
            salary: 30000.0 + (i as f64 * 100.0),
        })
        .collect();
    
    // 并行计算平均工资
    let start = Instant::now();
    let total_salary: f64 = people.par_iter().map(|p| p.salary).sum();
    let avg_salary = total_salary / people.len() as f64;
    println!("并行计算平均工资耗时: {:?}", start.elapsed());
    println!("平均工资: {:.2}", avg_salary);
    
    // 并行分组统计
    let start = Instant::now();
    let age_groups: Vec<(u32, usize)> = (20..70)
        .into_par_iter()
        .step_by(10)
        .map(|age_start| {
            let count = people
                .par_iter()
                .filter(|p| p.age >= age_start && p.age < age_start + 10)
                .count();
            (age_start, count)
        })
        .collect();
    
    println!("并行年龄分组统计耗时: {:?}", start.elapsed());
    for (age_start, count) in age_groups {
        println!("年龄 {}-{}: {} 人", age_start, age_start + 9, count);
    }
}

// 并行文件处理模拟
fn parallel_file_processing() {
    println!("\n=== 并行文件处理模拟 ===");
    
    // 模拟文件内容
    let file_contents: Vec<String> = (0..1000)
        .map(|i| format!("这是文件 {} 的内容，包含一些文本数据用于处理。", i))
        .collect();
    
    let processed_count = Arc::new(AtomicUsize::new(0));
    
    let start = Instant::now();
    let results: Vec<(usize, usize, usize)> = file_contents
        .par_iter()
        .map(|content| {
            let counter = processed_count.clone();
            
            // 模拟文件处理
            let word_count = content.split_whitespace().count();
            let char_count = content.chars().count();
            let line_count = content.lines().count();
            
            counter.fetch_add(1, Ordering::Relaxed);
            
            (word_count, char_count, line_count)
        })
        .collect();
    
    println!("并行文件处理耗时: {:?}", start.elapsed());
    println!("处理文件数: {}", processed_count.load(Ordering::Relaxed));
    
    let total_words: usize = results.iter().map(|(w, _, _)| w).sum();
    let total_chars: usize = results.iter().map(|(_, c, _)| c).sum();
    let total_lines: usize = results.iter().map(|(_, _, l)| l).sum();
    
    println!("总单词数: {}", total_words);
    println!("总字符数: {}", total_chars);
    println!("总行数: {}", total_lines);
}

// 并行数学计算示例
fn parallel_mathematical_computation() {
    println!("\n=== 并行数学计算 ===");
    
    let data: Vec<f64> = (0..1_000_000)
        .map(|i| i as f64 / 1000.0)
        .collect();
    
    // 并行计算复杂数学函数
    let start = Instant::now();
    let results: Vec<f64> = data
        .par_iter()
        .map(|&x| {
            // 复杂的数学计算
            let sin_val = x.sin();
            let cos_val = x.cos();
            let exp_val = (-x * x / 2.0).exp();
            sin_val * sin_val + cos_val * cos_val + exp_val
        })
        .collect();
    
    println!("并行数学计算耗时: {:?}", start.elapsed());
    
    // 并行统计
    let start = Instant::now();
    let (sum, min, max) = results
        .par_iter()
        .fold(
            || (0.0, f64::INFINITY, f64::NEG_INFINITY),
            |(sum, min, max), &val| (sum + val, min.min(val), max.max(val)),
        )
        .reduce(
            || (0.0, f64::INFINITY, f64::NEG_INFINITY),
            |(s1, min1, max1), (s2, min2, max2)| {
                (s1 + s2, min1.min(min2), max1.max(max2))
            },
        );
    
    println!("并行统计计算耗时: {:?}", start.elapsed());
    println!("总和: {:.6}", sum);
    println!("最小值: {:.6}", min);
    println!("最大值: {:.6}", max);
    println!("平均值: {:.6}", sum / results.len() as f64);
}

// 性能对比示例
fn performance_comparison() {
    println!("\n=== 性能对比 ===");
    
    let data: Vec<i32> = (0..1_000_000).collect();
    
    // 串行处理
    let start = Instant::now();
    let serial_sum: i64 = data.iter().map(|&x| (x as i64) * (x as i64)).sum();
    let serial_time = start.elapsed();
    
    // 并行处理
    let start = Instant::now();
    let parallel_sum: i64 = data.par_iter().map(|&x| (x as i64) * (x as i64)).sum();
    let parallel_time = start.elapsed();
    
    println!("串行处理耗时: {:?}", serial_time);
    println!("并行处理耗时: {:?}", parallel_time);
    println!("加速比: {:.2}x", serial_time.as_nanos() as f64 / parallel_time.as_nanos() as f64);
    println!("结果一致性: {}", serial_sum == parallel_sum);
}

fn main() {
    println!("Rayon 并行计算库示例");
    println!("=====================");
    
    // 显示系统信息
    println!("CPU 核心数: {}", num_cpus::get());
    println!("Rayon 线程池大小: {}", rayon::current_num_threads());
    
    // 运行各种示例
    basic_parallel_operations();
    parallel_sorting();
    parallel_data_processing();
    parallel_file_processing();
    parallel_mathematical_computation();
    performance_comparison();
    
    // 自定义快速排序示例
    println!("\n=== 自定义并行快速排序 ===");
    let mut test_data: Vec<i32> = (0..10000).rev().collect();
    let start = Instant::now();
    parallel_quicksort(&mut test_data);
    println!("自定义并行快速排序耗时: {:?}", start.elapsed());
    println!("排序正确性: {}", test_data.windows(2).all(|w| w[0] <= w[1]));
    
    println!("\n所有示例执行完成！");
}

// 添加 num_cpus 依赖的模拟（实际使用时需要在 Cargo.toml 中添加）
#[cfg(not(feature = "num_cpus"))]
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }
}