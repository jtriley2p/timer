use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use timer::*;

pub struct BenchAgent {
    pub value: usize,
}

impl Agent for BenchAgent {
    fn action(&mut self) {
        self.value += 1;
    }
}

fn slice_vector_bench<const N: usize>(agents: &mut [BenchAgent]) {
    let mut timer = SliceVectorTimer::<N>::new();

    for (i, agent) in agents.iter_mut().enumerate() {
        timer.start_timer(i % N, agent);
    }

    for _ in 0..N {
        timer.tick();
    }
}

fn vector_vector_bench(n: usize, agents: &mut [BenchAgent]) {
    let mut timer = VectorVectorTimer::new();

    for (i, agent) in agents.iter_mut().enumerate() {
        timer.start_timer(i % n, agent);
    }

    for _ in 0..n {
        timer.tick();
    }
}

fn slice_smallvector_bench<const M: usize, const N: usize>(agents: &mut [BenchAgent]) {
    let mut timer = SliceSmallVectorTimer::<M, N>::new();

    for (i, agent) in agents.iter_mut().enumerate() {
        timer.start_timer(i % N, agent);
    }

    for _ in 0..N {
        timer.tick();
    }
}

macro_rules! bench_group {
    ($name:ident, $m:expr, $n:expr) => {
        fn $name(c: &mut Criterion) {
            let mut agents: Vec<BenchAgent> = (0..($m * $n))
                .map(|_| BenchAgent {
                    value: black_box(420),
                })
                .collect();

            c.benchmark_group(stringify!($name))
                .bench_function("slice_vector_timer", |b| {
                    b.iter(|| slice_vector_bench::<$m>(&mut agents))
                })
                .bench_function("vector_vector_timer", |b| {
                    b.iter(|| vector_vector_bench(black_box($m), &mut agents))
                })
                .bench_function("slice_smallvector_timer", |b| {
                    b.iter(|| slice_smallvector_bench::<$m, $n>(&mut agents))
                });
        }
    };
}

bench_group!(group_1x1, 1, 1);
bench_group!(group_1x2, 1, 2);
bench_group!(group_1x256, 1, 256);
bench_group!(group_2x1, 2, 1);
bench_group!(group_2x2, 2, 2);
bench_group!(group_2x256, 2, 256);
bench_group!(group_256x1, 256, 1);
bench_group!(group_256x2, 256, 2);
bench_group!(group_256x256, 256, 256);

criterion_group!(
    benches,
    group_1x1,
    group_1x2,
    group_1x256,
    group_2x1,
    group_2x2,
    group_2x256,
    group_256x1,
    group_256x2,
    group_256x256,
);
criterion_main!(benches);
