use::std::fmt;

pub struct Monster {
    name: String,
    hp: i32,
    mp: i32,
    attack: i32,
}

impl fmt::Display for Monster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}\nHP: {}\nMP: {}\nattack: {}", self.name, self.hp, self.mp, self.attack)
    }
}

impl Monster {
    pub fn new() -> Monster {
        Monster {
            name : String::from("base_monster"),
            hp: 100,
            mp: 100,
            attack: 10,
        }
    }
}