pub struct IteratorTester<I> {
    inner: I,
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
