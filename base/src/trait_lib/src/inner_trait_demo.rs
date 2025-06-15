#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    progress: u8,
}

pub fn player() {
    let mut player = Player::default();
    println!("player:{:?}", player);
    player.name = String::from("xxx");
    println!("player:{:?}", player);
    player.progress = 3;
}
