#![feature(test)]

extern crate test;

pub struct IteratorTester<I: Iterator> {
    inner: I,
}
impl<I: Iterator> IteratorTester<I> {
    pub fn new(iter: I) -> Self {
        IteratorTester { inner: iter }
    }
}

impl<I: Iterator> Iterator for IteratorTester<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<I: Iterator> IteratorTester<I> {
    pub fn fold_mut<T, F>(mut self, init: T, mut f: F) -> T
    where
        F: FnMut(&mut T, I::Item),
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            f(&mut accum, x);
        }
        accum
    }

    pub fn fold_mut_fold<T, F>(self, init: T, mut f: F) -> T
    where
        F: FnMut(&mut T, I::Item),
    {
        self.fold(init, |mut accum, x| {
            f(&mut accum, x);
            accum
        })
    }

    pub fn fold_mut_each<T, F>(self, init: T, mut f: F) -> T
    where
        F: FnMut(&mut T, I::Item),
    {
        let mut accum = init;
        self.for_each(|x| {
            f(&mut accum, x);
        });
        accum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_works() {
        let tester = IteratorTester {
            inner: [1, 2, 3, 4, 5].iter(),
        };
        assert!(tester.eq([1, 2, 3, 4, 5].iter()))
    }

    #[test]
    fn fold_works() {
        let tester = IteratorTester {
            inner: [1, 2, 3, 4, 5].iter(),
        };
        assert_eq!(
            tester.fold(0, |sum, n| if n % 2 == 0 { sum + n } else { sum }),
            6
        )
    }

    #[test]
    fn fold_mut_works() {
        let tester = IteratorTester {
            inner: [1, 2, 3, 4, 5].iter(),
        };
        assert_eq!(
            tester.fold_mut(0, |sum, n| {
                if n % 2 == 0 {
                    *sum += n;
                }
            }),
            6
        )
    }

    #[test]
    fn fold_mut_fold_works() {
        let tester = IteratorTester {
            inner: [1, 2, 3, 4, 5].iter(),
        };
        assert_eq!(
            tester.fold_mut_fold(0, |sum, n| {
                if n % 2 == 0 {
                    *sum += n;
                }
            }),
            6
        )
    }

    #[test]
    fn fold_mut_each() {
        let tester = IteratorTester {
            inner: [1, 2, 3, 4, 5].iter(),
        };
        assert_eq!(
            tester.fold_mut_each(0, |sum, n| {
                if n % 2 == 0 {
                    *sum += n;
                }
            }),
            6
        )
    }
}

#[cfg(test)]
mod vec_benches {
    use super::*;
    use test::{black_box, Bencher};

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
        ($name_for:ident, $name_fold:ident, $name_fold_mut:ident, $name_fold_mut_fold:ident, $name_fold_mut_each:ident, $iter:expr) => {
            #[bench]
            fn $name_for(b: &mut Bencher) {
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
            }
            #[bench]
            fn $name_fold(b: &mut Bencher) {
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
            }
            #[bench]
            fn $name_fold_mut(b: &mut Bencher) {
                b.iter(|| {
                    IteratorTester::new($iter).fold_mut(Vec::new(), |nums, n| mut_closure!(nums, n))
                })
            }
            #[bench]
            fn $name_fold_mut_fold(b: &mut Bencher) {
                b.iter(|| {
                    IteratorTester::new($iter)
                        .fold_mut_fold(Vec::new(), |nums, n| mut_closure!(nums, n))
                })
            }
            #[bench]
            fn $name_fold_mut_each(b: &mut Bencher) {
                b.iter(|| {
                    IteratorTester::new($iter)
                        .fold_mut_each(Vec::new(), |nums, n| mut_closure!(nums, n))
                })
            }
        };
    }
    test!(
        bench_simple_for,
        bench_simple_fold,
        bench_simple_fold_mut,
        bench_simple_fold_mut_fold,
        bench_simple_fold_mut_each,
        (0i64..100_000).map(black_box)
    );
    test!(
        bench_chain_for,
        bench_chain_fold,
        bench_chain_fold_mut,
        bench_chain_fold_mut_fold,
        bench_chain_fold_mut_each,
        (0i64..50_000).chain(0..50_000).map(black_box)
    );
    test!(
        bench_flat_for,
        bench_flat_fold,
        bench_flat_fold_mut,
        bench_flat_fold_mut_fold,
        bench_flat_fold_mut_each,
        (0i64..100_000).map(std::iter::once).flat_map(black_box)
    );
}

#[cfg(test)]
mod num_benches {
    use super::*;
    use test::{black_box, Bencher};

    macro_rules! mut_closure {
        ($sum:ident, $n:ident) => {
            if $n % 3 == 0 {
                let n2 = $n / 3;
                if n2 % 3 == 0 {
                    *$sum += n2;
                }
            }
        };
    }

    macro_rules! test {
        ($name_for:ident, $name_fold:ident, $name_fold_mut:ident, $name_fold_mut_fold:ident, $name_fold_mut_each:ident, $iter:expr) => {
            #[bench]
            fn $name_for(b: &mut Bencher) {
                b.iter(|| {
                    let mut sum = 0;
                    for n in IteratorTester::new($iter) {
                        if n % 3 == 0 {
                            let n2 = n / 3;
                            if n2 % 3 == 0 {
                                sum += n2;
                            }
                        }
                    }
                    sum
                })
            }
            #[bench]
            fn $name_fold(b: &mut Bencher) {
                b.iter(|| {
                    IteratorTester::new($iter).fold(0, |sum, n| {
                        if n % 3 == 0 {
                            let n2 = n / 3;
                            if n2 % 3 == 0 {
                                return sum + n2;
                            }
                        }
                        sum
                    })
                })
            }
            #[bench]
            fn $name_fold_mut(b: &mut Bencher) {
                b.iter(|| IteratorTester::new($iter).fold_mut(0, |sum, n| mut_closure!(sum, n)))
            }
            #[bench]
            fn $name_fold_mut_fold(b: &mut Bencher) {
                b.iter(|| {
                    IteratorTester::new($iter).fold_mut_fold(0, |sum, n| mut_closure!(sum, n))
                })
            }
            #[bench]
            fn $name_fold_mut_each(b: &mut Bencher) {
                b.iter(|| {
                    IteratorTester::new($iter).fold_mut_each(0, |sum, n| mut_closure!(sum, n))
                })
            }
        };
    }
    test!(
        bench_simple_for,
        bench_simple_fold,
        bench_simple_fold_mut,
        bench_simple_fold_mut_fold,
        bench_simple_fold_mut_each,
        (0i64..100_000).map(black_box)
    );
    test!(
        bench_chain_for,
        bench_chain_fold,
        bench_chain_fold_mut,
        bench_chain_fold_mut_fold,
        bench_chain_fold_mut_each,
        (0i64..50_000).chain(0..50_000).map(black_box)
    );
    test!(
        bench_flat_for,
        bench_flat_fold,
        bench_flat_fold_mut,
        bench_flat_fold_mut_fold,
        bench_flat_fold_mut_each,
        (0i64..100_000).map(std::iter::once).flat_map(black_box)
    );
}

#[cfg(test)]
mod tuple_benches {
    use super::*;
    use test::{black_box, Bencher};

    macro_rules! mut_closure {
        ($nums:ident, $n:ident) => {
            if $n % 3 == 0 {
                let n2 = $n / 3;
                if n2 % 3 == 0 {
                    $nums[n2 as usize % $nums.len()] += n2;
                }
            }
        };
    }

    macro_rules! test {
        ($name_for:ident, $name_fold:ident, $name_fold_mut:ident, $name_fold_mut_fold:ident, $name_fold_mut_each:ident, $iter:expr) => {
            #[bench]
            fn $name_for(b: &mut Bencher) {
                b.iter(|| {
                    let mut nums = [0i64; 10];
                    for n in IteratorTester::new($iter) {
                        if n % 3 == 0 {
                            let n2 = n / 3;
                            if n2 % 3 == 0 {
                                nums[n2 as usize % nums.len()] += n2;
                            }
                        }
                    }
                    nums
                })
            }
            #[bench]
            fn $name_fold(b: &mut Bencher) {
                b.iter(|| {
                    IteratorTester::new($iter).fold([0i64; 10], |mut nums, n| {
                        if n % 3 == 0 {
                            let n2 = n / 3;
                            if n2 % 3 == 0 {
                                nums[n2 as usize % nums.len()] += n2;
                            }
                        }
                        nums
                    })
                })
            }
            #[bench]
            fn $name_fold_mut(b: &mut Bencher) {
                b.iter(|| {
                    IteratorTester::new($iter).fold_mut([0i64; 10], |nums, n| mut_closure!(nums, n))
                })
            }
            #[bench]
            fn $name_fold_mut_fold(b: &mut Bencher) {
                b.iter(|| {
                    IteratorTester::new($iter)
                        .fold_mut_fold([0i64; 10], |nums, n| mut_closure!(nums, n))
                })
            }
            #[bench]
            fn $name_fold_mut_each(b: &mut Bencher) {
                b.iter(|| {
                    IteratorTester::new($iter)
                        .fold_mut_each([0i64; 10], |nums, n| mut_closure!(nums, n))
                })
            }
        };
    }
    test!(
        bench_simple_for,
        bench_simple_fold,
        bench_simple_fold_mut,
        bench_simple_fold_mut_fold,
        bench_simple_fold_mut_each,
        (0i64..100_000).map(black_box)
    );
    test!(
        bench_chain_for,
        bench_chain_fold,
        bench_chain_fold_mut,
        bench_chain_fold_mut_fold,
        bench_chain_fold_mut_each,
        (0i64..50_000).chain(0..50_000).map(black_box)
    );
    test!(
        bench_flat_for,
        bench_flat_fold,
        bench_flat_fold_mut,
        bench_flat_fold_mut_fold,
        bench_flat_fold_mut_each,
        (0i64..100_000).map(std::iter::once).flat_map(black_box)
    );
}
