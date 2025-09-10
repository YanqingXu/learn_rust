use criterion::{black_box, criterion_group, criterion_main, Criterion};
use learn_rust::utils::Timer;

fn benchmark_timer(c: &mut Criterion) {
    c.bench_function("timer creation", |b| {
        b.iter(|| {
            let _timer = Timer::new(black_box("test"));
        })
    });
}

fn benchmark_fibonacci(c: &mut Criterion) {
    fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 1,
            1 => 1,
            n => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    c.bench_function("fibonacci 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, benchmark_timer, benchmark_fibonacci);
criterion_main!(benches);