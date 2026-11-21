use crate::healthsys;
use crate::items;
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

pub fn playerdamage(player: &mut Player) {
    player.damage = match player.item.as_str() {
        "iron sword" => items::SWORD.damage,
        "bow" => items::BOW.damage,
        _ => {
            println!("bad text :(");
            player.damage
        }
    };
}
