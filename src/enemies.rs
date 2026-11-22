use rand::prelude::IndexedRandom;

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

impl Enemystats {
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}

pub fn creaturespawn() -> Enemystats {
    let templates = [SKULL, GHOST, REDDIT_CHUD, WENDYS_MANAGER];
    let mut rng = rand::rng();
    *templates.choose(&mut rng).unwrap()
}
