use criterion::{black_box, criterion_group, criterion_main, Criterion};

use two_sets::fast_solution;
use two_sets::slow_solution;

fn benchmark_fast(c: &mut Criterion) {
    c.bench_function("fast_O_n", |b| {
        b.iter(|| {
            fast_solution::solve(black_box(100000));
        })
    });
}

fn benchmark_slow(c: &mut Criterion) {
    c.bench_function("slow_O_n2", |b| {
        b.iter(|| {
            slow_solution::solve(black_box(5000));
        })
    });
}

criterion_group!(benches, benchmark_fast, benchmark_slow);
criterion_main!(benches);