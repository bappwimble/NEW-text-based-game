pub use std;
pub use std::io;

pub struct Weapon {
    pub name: &'static str,
    //pub id: i32,
    pub damage: i32,
    //pub weight: i32,
    //pub range: i32,
}
pub const SWORD: Weapon = Weapon {
    name: "iron sword",
    //id: 1,
    damage: 15,
    //weight: 9,
    //range: 4,
};
pub const BOW: Weapon = Weapon {
    name: "bow",
    //id: 2,
    damage: 10,
    //weight: 4,
    //range: 20,
};

pub fn itemselect() -> String {
    let mut input = String::new();

    println!("please select item: \n");
    io::stdin().read_line(&mut input).expect("no weapon?");

    let weapon_choice = match input.trim().to_lowercase().as_str() {
        "1" | "sword" | "iron sword" => SWORD.name.to_string(),
        "2" | "bow" => BOW.name.to_string(),
        _ => {
            println!("bad text :(");
            String::new()
        }
    };
    weapon_choice
}
