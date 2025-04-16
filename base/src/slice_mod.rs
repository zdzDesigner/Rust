pub struct Mqtt {
    fd_list: Vec<u32>,
}

impl Mqtt {
    pub fn new(list: &[u32]) -> Mqtt {
        Mqtt {
            fd_list: list.to_vec(),
        }
    }

    pub fn print(&self) {
        for &item in self.fd_list.iter() {
            println!("item:{}", item);
        }
    }
}

#[cfg(test)]
mod test_slice {}
