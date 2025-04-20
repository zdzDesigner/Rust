// 关联类型测试
#[cfg(test)]
mod test_associated {

    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
        // fn current(&self) -> Self::Item;
    }

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Self { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    #[test]
    fn base() {
        let mut counter = Counter { count: 0 };
        println!("{}", counter.count);
        println!("{}", counter.next().unwrap());
        println!("{}", counter.next().unwrap());
        println!("{}", counter.next().unwrap());
        println!("{}", counter.next().unwrap());
        println!("{}", counter.next().unwrap());
        println!("{}", counter.next().unwrap_or(8));
        let counter = Counter { count: 0 };
        println!("{}", counter.count);
    }
}
#[cfg(test)]
mod test_std_trait {

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Self { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    #[test]
    fn base() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        println!("sum:{}", sum);
    }
}
