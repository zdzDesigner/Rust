#[cfg(test)]
mod deconstruction {

    struct Point {
        x: i32,
        y: i32,
    }
    #[test]
    fn deconstruct_struct() {
        let p = Point { x: 10, y: 20 };

        let Point { x: a, y: b } = p;
        println!("x:{a}, y:{b}");

        let Point { x, y } = p;
        println!("x:{x}, y:{y}");
    }
}
