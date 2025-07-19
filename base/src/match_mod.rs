#[cfg(test)]
mod test_match {
    #[test]
    fn match_some() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("not match Got 50"),
            Some(5) => println!("Matched Got 5"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);
        match x {
            Some(v) => println!("Matched, v = {:?}", v),
            _ => println!("Default case, x = {:?}", x),
        }
    }

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    #[test]
    fn match_deconstruction() {
        let p = Point { x: 10, y: 20 };
        match p {
            Point { x: 10, y: 20 } => println!("hint x,y"),
            _ => println!("default ...."),
        }

        match p {
            Point { x: 10, y } => println!("just hint x"),
            _ => println!("default ...."),
        }
        match p {
            Point { x, y } => println!("hint Point wrap"),
            _ => println!("default ...."),
        }
    }

    #[test]
    fn some_underline() {
        let s = Some(String::from("zdz"));

        if let Some(_) = s {
            println!("assign value {s:?}"); // _ not move
        }

        if let Some(_s) = s {
            println!("assign value {_s}"); // move
        }
        // println!("{s:?}"); // move
    }

    #[test]
    fn unconstruction_point_point() {
        let p = Point { x: 10, y: 30 };
        let Point { x, .. } = p;
        println!("unconstruction {x}");
        println!("{p:?}");
    }

    #[test]
    fn mid_point_point() {
        let vals = (1, 3, 4, 8, 9);

        match vals {
            (first, .., last) => {
                println!("first:{first}, last:{last}")
            }
        }
    }

    #[test]
    fn match_condition() {
        let val = Some(4);

        match val {
            Some(x) if (x > 5) => println!(">5"),
            Some(x) => println!("x:{x}"), // 命中
            _ => {}
        }
    }
}
