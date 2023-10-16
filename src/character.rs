use std::cmp;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct CharacterParameters {
    pub name: String,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub defensing: bool,
}

impl CharacterParameters {
    pub fn new(name: String, hp: i32, attack: i32, defense: i32) -> CharacterParameters {
        CharacterParameters {
            name,
            hp,
            attack,
            defense,
            defensing: false,
        }
    }
    pub fn attack(&self) -> i32 {
        self.attack
    }

    pub fn defense(&self) -> i32 {
        self.defense
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn get_hp(&self) -> i32 {
        self.hp
    }
    pub fn set_hp(&mut self, hp: i32) {
        self.hp = hp;
    }
    pub fn defensing(&self) -> bool {
        self.defensing
    }
    pub fn set_defensing(&mut self, defensing: bool) {
        self.defensing = defensing;
    }
    pub fn attack_to<T: HasCharacterParameters>(&self, target: &mut T) {
        let random_range = rand::thread_rng().gen_range(-5..5);
        let damage = if target.params().defensing {
            self.attack() - target.params().defense * 2 + random_range
        } else {
            self.attack() - target.params().defense + random_range
        };
        if damage > 0 {
            target.damaged(damage);
        } else {
            println!("{}はダメージを受けていない！", target.params().name);
        }
    }
}

pub trait HasCharacterParameters {
    fn params(&mut self) -> &mut CharacterParameters;
    fn damaged(&mut self, damage: i32) {
        let params = self.params();
        print!("{}は{}のダメージを受けた！\n", params.name, damage);
        params.hp = cmp::max(params.hp - damage, 0);
    }
}

#[derive(Clone, Debug)]
pub struct Monster {
    params: CharacterParameters,
}

impl HasCharacterParameters for Monster {
    fn params(&mut self) -> &mut CharacterParameters {
        &mut self.params
    }
}

#[derive(Clone, Debug)]
pub struct Hero {
    params: CharacterParameters,
}

impl HasCharacterParameters for Hero {
    fn params(&mut self) -> &mut CharacterParameters {
        &mut self.params
    }
}

impl Monster {
    pub fn new(name: &str, hp: i32, attack: i32, defense: i32) -> Monster {
        let params = CharacterParameters {
            name: name.to_string(),
            hp,
            attack,
            defense,
            defensing: false,
        };
        Monster {
            params,
        }
    }
}

impl Hero {
    pub fn new(name: &str, hp: i32, attack: i32, defense: i32) -> Hero {
        let params = CharacterParameters {
            name: name.to_string(),
            hp,
            attack,
            defense,
            defensing: false,
        };
        Hero {
            params,
        }
    }
}
