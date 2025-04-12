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

    #[test]
    #[should_panic]
    fn shoud_pinic() {
        panic!("panic!!")
    }
}

#[cfg(test)]
mod sest_lifetime_generics {

    trait Enver<'a> {
        fn isDev(&self) -> bool;
        fn message(&self) -> &'a str;
    }

    #[derive(Debug)]
    struct Build<'a> {
        env: bool,
        vender: &'a str,
    }
    impl<'a> Enver<'a> for Build<'a> {
        fn isDev(&self) -> bool {
            println!("{:?}", self.message());
            return self.env;
        }
        fn message(&self) -> &'a str {
            return self.vender;
        }
    }

    #[test]
    fn test_env() {
        let build = Build {
            env: true,
            vender: "Inforbit",
        };
        println!("{:#?}", build);
        println!("isdev:{}", build.isDev());
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

#[derive(Debug)]
struct Tcp<'a> {
    ip: &'a IPV4<'a>,
    port: &'a str,
}

#[cfg(test)]
mod test_struct_lifetime {
    use super::*;

    #[test]
    fn test_nest_struct() {
        let ipv4 = IPV4 { ip: "127.0.0.1" };
        println!("ipv4.ip:{}", ipv4.ip);
        println!("ipv4:{:?}", ipv4);

        println!("version:{}", ipv4.version());

        let tcp = Tcp {
            ip: &ipv4,
            port: "3333",
        };
        println!("tcp:{:#?}", tcp);
        println!("ip: {}", tcp.ip.ip);
    }
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
