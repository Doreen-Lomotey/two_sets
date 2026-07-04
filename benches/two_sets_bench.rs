use criterion::{black_box, criterion_group, criterion_main, Criterion};

use two_sets::fast_solution;
use two_sets::slow_solution;

/// BENCHMARK INTERPRETATION:
/// Fast = greedy subset sum approach (O(n), early termination possible)
/// Slow = brute-force nested scanning approach (O(n²))
/// This shows how algorithm design impacts runtime even for the same problem.
fn benchmark_fast(c: &mut Criterion) {
    c.bench_function("fast_O_n", |b| {
        b.iter(|| {
            fast_solution::solve(black_box(100000));
        })
    });
}

/// BENCHMARK INTERPRETATION:
/// Fast = optimized greedy solution (single pass logic)
/// Slow = repeated scanning with nested loops (inefficient simulation)
/// Demonstrates practical performance gap between O(n) and O(n²)
fn benchmark_slow(c: &mut Criterion) {
    c.bench_function("slow_O_n2", |b| {
        b.iter(|| {
            slow_solution::solve(black_box(5000));
        })
    });
}

criterion_group!(benches, benchmark_fast, benchmark_slow);
criterion_main!(benches);