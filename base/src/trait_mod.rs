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
}
