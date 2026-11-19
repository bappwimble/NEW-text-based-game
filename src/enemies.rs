use std;

#[derive(Clone, Copy)]
pub struct Enemystats {
    pub title: &'static str,
    pub health: i32,
    pub enemydamage: i32,
}

pub const SKULL: Enemystats = Enemystats {
    title: "skullton",
    health: 50,
    enemydamage: 12,
};

pub const GHOST: Enemystats = Enemystats {
    title: "spooky ghost",
    health: 25,
    enemydamage: 2,
};

pub const REDDIT_CHUD: Enemystats = Enemystats {
    title: "reddit chud",
    health: 100,
    enemydamage: 5,
};

pub const WENDYS_MANAGER: Enemystats = Enemystats {
    title: "ol' gill",
    health: 100,
    enemydamage: 8,
};

pub const ALL_ENEMIES: [Enemystats; 4] = [SKULL, GHOST, REDDIT_CHUD, WENDYS_MANAGER];
