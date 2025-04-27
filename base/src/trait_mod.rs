pub trait Summary {
    fn summarize(&self) -> String;
    fn def(&self) -> u8 {
        1
    }
}

struct House {
    rooms: u8,
    descr: String,
}

impl Summary for House {
    fn summarize(&self) -> String {
        format!(
            "rooms num is:{}, describe the house is:{}",
            self.rooms, self.descr
        )
    }
    fn def(&self) -> u8 {
        8
    }
}

fn notify(summary: impl Summary) -> String {
    summary.summarize()
}

fn notify_t<T: Summary>(summary: T) -> String {
    summary.summarize()
}

#[cfg(test)]
mod test_trait {
    use super::*;
    #[test]
    fn test_trait() {
        let house = House {
            rooms: 3,
            descr: String::from("so big"),
        };
        println!("house.summarize:{}", house.summarize());
        println!("house.def:{}", house.def());

        // println!("notify:{}", notify(house));
        println!("notify_t:{}", notify_t(house));
    }

    #[test]
    fn call_trait_fn() {
        let house = House {
            rooms: 10,
            descr: String::from("vvv"),
        };

        println!("{}", house.def());
        println!("force call trait fn:{}", <House as Summary>::def(&house));
    }

    use trait_lib::duplicate::{run_addto, run_debug_show, run_dupli, run_min};
    use trait_lib::inner_trait_demo::player;
    use trait_lib::logger::run_log;
    use trait_lib::share_demo::shape;
    use trait_lib::supper_demo::supper_lib::create;

    #[test]
    fn test_trait_lib() {
        create();
        shape();
        player();
        run_log();
        run_dupli();
        run_addto();
        run_debug_show();
        run_min();
    }
}
