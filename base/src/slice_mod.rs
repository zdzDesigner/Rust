pub struct Mqtt {
    fd_list: Vec<u32>,
}

impl Mqtt {
    pub fn new(list: &[u32]) -> Mqtt {
        Mqtt {
            fd_list: list.to_vec(),
        }
    }

    pub fn print(&self) {
        for &item in self.fd_list.iter() {
            println!("item:{}", item);
        }
    }
}

#[cfg(test)]
mod test_slice {}
mod test_slice_vec {
    #[test]
    fn slice_vec() {
        let list: [i32; 4] = [1, 3, 4, 8];
        // 在表达式`ele + 1`中，由于`i32`是基本类型，实现了`Copy`，所以这里会先解引用`(*ele)`得到`i32`（复制），然后加1。所以这里会复制值，并生成一个新的值。
        println!("{:?}", &list.iter().map(|ele| ele + 1).collect::<Vec<i32>>());
        println!("{:?}", &list.iter().collect::<Vec<_>>());
        // `<Vec<&i32>>`:`iter()`返回的是`引用`迭代器，因为数组nums是一个固定大小的数组，存储在栈上。我们通过引用来访问其元素以*避免所有权的转移*（数组本身的所有权没有被消耗，因此可以重复使用）。
        println!("{:?}", &list.iter().collect::<Vec<&i32>>());
    }
}
