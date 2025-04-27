trait Animal {
    fn leg_const(&self) -> usize;
}

// bound(约束) 必须要实现Animal trait
trait Pet: Animal {
    fn name(&self) -> String;
}

struct Dog(String);

impl Pet for Dog {
    fn name(&self) -> String {
        self.0.clone()
    }
}
impl Animal for Dog {
    fn leg_const(&self) -> usize {
        4
    }
}

pub mod supper_lib {
    use super::*;

    pub fn create() {
        let pet = Dog(String::from("big yellow"));
        println!("{}", pet.name());
        println!("{}", pet.leg_const());
    }
}
