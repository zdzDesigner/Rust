use std::io::{self, Write};

pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches fruit"),
            }
        }
        pub fn fruit(&self) -> String {
            self.seasonal_fruit.clone()
        }
    }
}

pub fn eat_at_restaurant() {
    // 在夏天点一份黑麦面包作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 更改我们想要的面包
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    println!("{}", meal.fruit());

    // 如果取消下一行的注释，将会导致编译失败；我们不被允许
    // 看到或更改随餐搭配的季节水果
    // meal.seasonal_fruit = String::from("blueberries");

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    // stdout.write(String::from("xxxxxx").as_bytes());
    let _ = handle.write_all(b"xxxxxx\n");
}
