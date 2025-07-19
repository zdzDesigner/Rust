#[cfg(test)]
mod mut_mod {
    #[test]
    fn mut_demo1() {
        {
            let r = 5; // copy
            let r1 = r;
            println!("{r}");
            println!("{r1}");
        }
        {
            let r = 5;
            let r1 = &r;
            println!("{r}");
            println!("{r1}");
        }

        {
            let mut r = 5;
            let r1 = &mut r;
            // r = 6;
            *r1 = 8;
            println!("r:{r}");
            // println!("r1:{r1}"); // error
        }
    }
}
