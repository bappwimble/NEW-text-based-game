use std::io::{self, Write};

mod items;
use items::itemselect;

mod healthsys;

mod basic_functions;
use basic_functions::*;

mod player;
use player::*;

mod enemies;
use enemies::*;

fn playersetup() -> player::Player {
    let mut player = playertitle();

    playertitle();

    player.item = itemselect();

    player.damage = match player.item.as_str() {
        "iron sword" => items::SWORD.damage,
        "bow" => items::BOW.damage,
        _ => {
            println!("bad text :(");
            player.damage
        }
    };

    player
}

fn main() {
    let player = playersetup();

    loop {
        println!(
            "\nplayer: {} has {} health and is currently wielding: {}\nthey do {} per hit",
            player.name, player.health, player.item, player.damage
        );

        creaturespawn();

        loop {
            damageloop();
        }
    }
}
