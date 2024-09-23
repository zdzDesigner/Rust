pub struct Mqtt {
    fd_list: &[u32],
}

impl Mqtt {
    pub fn new(list: &[u32]) -> Mqtt {
        Mqtt { fd_list: list.clone() }
    }

    pub fn print(&self) {
        for &item in self.fd_list.iter() {
            println!("item:{}", item);
        }
    }
}
