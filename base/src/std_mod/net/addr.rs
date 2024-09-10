use std::net::IpAddr;

pub fn parse() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("home:{}", home);

    match home {
        std::net::IpAddr::V4(val) => println!("ip v4:{}", val),
        std::net::IpAddr::V6(val) => println!("ip v6:{}", val),
    };
    let val = match home {
        std::net::IpAddr::V4(_) => 4,
        std::net::IpAddr::V6(_) => 6,
    };

    println!("val:{}", val);
}
