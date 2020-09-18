use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fold_mut::*;
use std::iter::once;

macro_rules! mut_closure {
    ($nums:ident, $n:ident) => {
        if $n % 3 == 0 {
            let n2 = $n / 3;
            if n2 % 3 == 0 {
                $nums.push(n2);
            }
        }
    };
}

macro_rules! test {
    ($criterion:expr, $name:literal, $iter:expr) => {
        let mut group = $criterion.benchmark_group($name);
        group.bench_function("for", |b| {
            b.iter(|| {
                let mut nums = Vec::new();
                for n in IteratorTester::new($iter) {
                    if n % 3 == 0 {
                        let n2 = n / 3;
                        if n2 % 3 == 0 {
                            nums.push(n2);
                        }
                    }
                }
                nums
            })
        });
        group.bench_function("fold", |b| {
            b.iter(|| {
                IteratorTester::new($iter).fold(Vec::new(), |mut nums, n| {
                    if n % 3 == 0 {
                        let n2 = n / 3;
                        if n2 % 3 == 0 {
                            nums.push(n2);
                        }
                    }
                    nums
                })
            })
        });
        group.bench_function("fold_mut", |b| {
            b.iter(|| {
                IteratorTester::new($iter).fold_mut(Vec::new(), |nums, n| mut_closure!(nums, n))
            })
        });
        group.bench_function("fold_mut_fold", |b| {
            b.iter(|| {
                IteratorTester::new($iter)
                    .fold_mut_fold(Vec::new(), |nums, n| mut_closure!(nums, n))
            })
        });
        group.bench_function("fold_mut_each", |b| {
            b.iter(|| {
                IteratorTester::new($iter)
                    .fold_mut_each(Vec::new(), |nums, n| mut_closure!(nums, n))
            })
        });
        group.finish();
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    test!(c, "simple iter", (0i64..100_000).map(black_box));
    test!(
        c,
        "chain iter",
        (0i64..50_000).chain(0..50_000).map(black_box)
    );
    test!(
        c,
        "flat iter",
        (0i64..100_000).map(once).flat_map(black_box)
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
