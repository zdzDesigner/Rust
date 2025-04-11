#[cfg(test)]
mod test_sample {
    fn base() {
        let name_wrap = String::from("zdz too long");
        let res = longest(&name_wrap, "aa");
        println!("lifetime:{}", res);

        // // 声明周期问题
        // let res;
        // {
        //     let name_inner = String::from("designer");
        //     res = longest(&name_wrap.as_str(), &name_inner);
        // }
        // println!("lifetime:{}", res);
    }

    fn longest<'a>(wrap: &'a str, inner: &'a str) -> &'a str {
        if wrap.len() > inner.len() {
            wrap
        } else {
            inner
        }
    }
    #[test]
    fn lifetime_base() {
        base();
    }
}

#[derive(Debug)]
struct IPV4<'a> {
    ip: &'a str,
}

impl<'a> IPV4<'a> {
    fn version(&self) -> i32 {
        return 3;
    }
}

fn struct_test() {
    let ips = String::from("127.0.0.1/127.0.0.2/127.0.0.3/127.0.0.4");
    let res = ips.split("/").next().expect("sss");
    println!("res:{}", res);
    let ipv4 = IPV4 { ip: res };
    println!("ipv4.ip:{}", ipv4.ip);
    println!("ipv4:{:?}", ipv4);

    println!("version:{}", ipv4.version());

    let tcp = Tcp {
        ip: ipv4,
        port: "3333",
    };
    println!("tcp:{:?}", tcp);
}

#[derive(Debug)]
struct Tcp<'a> {
    ip: IPV4<'a>,
    port: &'a str,
}

#[cfg(test)]
mod lifetime_test {

    fn max<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut max_item = list[0];
        for &item in list.iter() {
            if item > max_item {
                max_item = item;
            }
        }
        return max_item;
    }

    #[test]
    fn max_test() {
        let v = max(&vec![4, 1, 9]);
        println!("max:{}", v);
    }

    fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        return a + b;
    }
    #[test]
    fn add_test() {
        let v = add(4, 2);
        println!("add:{}", v);
    }
}
