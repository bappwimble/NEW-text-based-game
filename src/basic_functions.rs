use crate::enemies::{self, ALL_ENEMIES, Enemystats};
use crate::player::Player;
use rand::Rng;
use std::io;

pub fn statsdisplay(player: &Player) {
    println!(
        "\nplayer: {} has {} health and is currently wielding: {}\nthey do {} per hit",
        player.name, player.health, player.item, player.damage
    );
}

pub fn creaturespawn() -> Enemystats {
    let mut rng = rand::rng();
    let index = rng.random_range(0..ALL_ENEMIES.len());

    enemies::ALL_ENEMIES[index]
}

pub fn damageloop() {
    let mut input = String::new();
    let current_enemy = creaturespawn();

    println!(
        "{} spawns!\nit has {} health, and does {} damage per hit.",
        current_enemy.title, current_enemy.health, current_enemy.enemydamage
    );

    println!("\nwaiting...\n\n");
    io::stdin().read_line(&mut input).expect("no text?");

    if input.trim() == "hit" {
        println!("goog");
    } else if input.trim().is_empty() {
        println!("?");
    } else {
        return;
    }
}
