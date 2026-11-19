use crate::healthsys;
pub use std::io;

pub struct Player {
    pub name: String,
    pub health: i32,
    pub item: String,
    pub damage: i32,
    //inventory: String,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            health: healthsys::health(),
            item: String::new(),
            damage: 0,
            //inventory: String::new(),
        }
    }
}

pub fn playertitle() -> Player {
    let mut player = Player::new();

    println!("please enter name: \n");
    io::stdin().read_line(&mut player.name).expect("no name?");
    player.name = player.name.trim().to_string();

    player
}
