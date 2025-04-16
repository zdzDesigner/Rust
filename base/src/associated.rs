// 关联类型测试
#[cfg(test)]
mod test_associated {

    trait Iter {
        type Item;
        fn get(&self) -> Self::Item;
    }

    #[test]
    fn base() {}
}
