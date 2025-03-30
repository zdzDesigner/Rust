pub fn base() {
    let name_wrap = String::from("zdz too long");
    let res = longest(&name_wrap, "aa");
    println!("lifetime:{}", res);

    // 声明周期问题
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

    let tcp = Tcp{ip:ipv4, port:"3333"};
    println!("tcp:{:?}", tcp);
}

#[derive(Debug)]
struct Tcp<'a> {
    ip: IPV4<'a>,
    port: &'a str,
}


#[cfg(test)]
mod lifetime_test {
    use super::*;

    #[test]
    fn lifetime_base() {
        base();
        struct_test();
    }
}
