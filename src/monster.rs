use::std::fmt;

#[derive(Clone)]
pub enum State {
    Live,
    Sleep,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Live => write!(f, "存活"),
            Self::Sleep => write!(f, "睡着"),
        }
    }
}

pub struct Monster {
    name: String,
    hp: i32,
    max_hp: i32,
    mp: i32,
    max_mp: i32,
    attack: i32,
    state: State,
}

impl fmt::Display for Monster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "HP: {}/{}", self.hp, self.max_hp)?;
        writeln!(f, "MP: {}/{}", self.mp, self.max_mp)?;
        writeln!(f, "attack: {}", self.attack)?;
        write!(f, "State: {}",self.state)
    }
}

impl Monster {
    pub fn new() -> Monster {
        Monster {
            name : String::from("base_monster"),
            max_hp: 100,
            max_mp: 100,
            hp: 100,
            mp: 100,
            attack: 10,
            state: State::Live,
        }
    }

    pub fn build(name: String, hp: i32, mp: i32, attack: i32) -> Monster {
        Monster {
            name,
            max_hp: hp,
            max_mp: mp,
            hp,
            mp,
            attack,
            state: State::Live,
        }
    }

    pub fn build_from(monster: &Monster) -> Monster {
        Monster {
            name: monster.name.clone(),
            state: monster.state.clone(),
            hp: monster.hp,
            mp: monster.mp,
            max_hp: monster.hp,
            max_mp: monster.mp,
            attack: monster.attack,
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

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn normal_attack(&self, target: &mut Self) -> (i32, i32) {
        target.hp -= self.attack;

        if target.hp <= 0 {
            target.state = State::Sleep;
        }

        (self.attack, target.hp)
    }

    pub fn power_attack(&self, target: &mut Self) -> (i32, i32) {
        target.hp -= self.attack*2;

        if target.hp <= 0 {
            target.state = State::Sleep;
        }

        (self.attack*2, target.hp)
    }
}