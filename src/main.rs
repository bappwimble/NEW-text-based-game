mod basic_functions;
mod enemies;
mod healthsys;
mod items;
mod player;
use basic_functions::*;
use items::itemselect;
use player::*;

fn playersetup() -> player::Player {
    let mut player = playertitle();

    player.item = itemselect();
    player::playerdamage(&mut player);

    player
}

fn main() {
    let player = playersetup();
    basic_functions::statsdisplay(&player);

    loop {
        damageloop(&player);
    }
}
