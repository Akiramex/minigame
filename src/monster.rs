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

    pub fn build(name: String, hp: i32, mp: i32, attack: i32) -> Monster {
        Monster {
            name,
            hp,
            mp,
            attack
        }
    }

    pub fn attack(&self) -> i32 {
        self.attack
    }

    pub fn hp(&self) -> i32 {
        self.hp
    }

    pub fn mp(&self) -> i32 {
        self.mp
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn normal_attack(&self, target: &mut Self) -> (i32, i32) {
        target.hp -= self.attack;
        (self.attack, target.hp)
    }

    pub fn power_attack(&self, target: &mut Self) -> (i32, i32) {
        target.hp -= self.attack*2;
        (self.attack, target.hp)
    }
}