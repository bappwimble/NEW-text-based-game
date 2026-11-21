use crate::enemies::{Enemystats, creaturespawn};
use crate::player::Player;
use std::io;

pub fn statsdisplay(player: &Player) {
    println!(
        "\nplayer: {} has {} health and is currently wielding: {}\nthey do {} per hit",
        player.name, player.health, player.item, player.damage
    );
}

pub fn announce_enemy(enemy: &Enemystats) {
    println!(
        "\n{} spawns!\nit has {} health, and does {} damage per hit.",
        enemy.title, enemy.health, enemy.enemydamage
    );
}

pub fn damage(player: &Player, enemy: &mut Enemystats) {
    enemy.health -= player.damage;

    println!(
        "{} takes '{}' damage! {} is now at {} health.",
        enemy.title, player.damage, enemy.title, enemy.health,
    )
}

pub fn damageloop(player: &Player) {
    let mut input = String::new();
    let mut current_enemy = creaturespawn();
    announce_enemy(&current_enemy);

    loop {
        println!("\nwaiting for input...\n\n");
        io::stdin().read_line(&mut input).expect("no text?");

        if input.trim() == "hit" {
            damage(player, &mut current_enemy);
        } else if input.trim().is_empty() {
            println!("?");
        } else {
            continue;
        }
    }
}
