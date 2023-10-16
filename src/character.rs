use std::{cmp, collections::HashMap};
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
    exp: i32,
}

impl HasCharacterParameters for Monster {
    fn params(&mut self) -> &mut CharacterParameters {
        &mut self.params
    }
}

#[derive(Clone, Debug)]
pub struct Hero {
    params: CharacterParameters,
    exp: i32,
    level: i32,
}


pub trait HasLevel {
    fn level(&self) -> i32;
    fn exp(&self) -> i32;
}

impl HasLevel for Hero {
    fn level(&self) -> i32 {
        self.level
    }
    fn exp(&self) -> i32 {
        self.exp
    }
}

impl HasCharacterParameters for Hero {
    fn params(&mut self) -> &mut CharacterParameters {
        &mut self.params
    }
}

impl Monster {
    pub fn new(name: &str, hp: i32, attack: i32, defense: i32, exp: i32) -> Monster {
        let params = CharacterParameters {
            name: name.to_string(),
            hp,
            attack,
            defense,
            defensing: false,
        };
        Monster {
            params,
            exp,
        }
    }

    pub fn get_exp(&self) -> i32 {
        self.exp
    }
}

impl Hero {
    fn exp_table() -> HashMap<i32, i32> {
        HashMap::from([
            (1,  0),
            (2,  2),
            (3,  5),
            (4,  9),
            (5,  14),
            (6,  20),
            (7,  27),
            (8,  35),
            (9,  44),
            (10, 54),
        ])
    }

    fn level_table() -> HashMap<i32, CharacterParameters> {
        HashMap::from([
            (1,  CharacterParameters::new(String::from("勇者"), 100, 10,  5)),
            (2,  CharacterParameters::new(String::from("勇者"), 110, 11,  6)),
            (3,  CharacterParameters::new(String::from("勇者"), 120, 12,  7)),
            (4,  CharacterParameters::new(String::from("勇者"), 130, 13,  8)),
            (5,  CharacterParameters::new(String::from("勇者"), 140, 14,  9)),
            (6,  CharacterParameters::new(String::from("勇者"), 150, 15, 10)),
            (7,  CharacterParameters::new(String::from("勇者"), 160, 16, 11)),
            (8,  CharacterParameters::new(String::from("勇者"), 170, 17, 12)),
            (9,  CharacterParameters::new(String::from("勇者"), 180, 18, 13)),
            (10, CharacterParameters::new(String::from("勇者"), 190, 19, 14)),
        ])
    }

    pub fn new() -> Hero {
        let level = 1;
        let params = Hero::level_table().get(&level).unwrap().clone();
        Hero {
            params,
            exp: 0,
            level,
        }
    }

    pub fn check_level_up(&self, exp_got: i32) -> i32 {
        let mut level = 1;
        for (l, exp) in Hero::exp_table() {
            if exp_got >= exp {
                level = l;
            }
        }
        level
    }

    pub fn level_up(&mut self, level: i32) {
        let params = Hero::level_table().get(&level).unwrap().clone();
        self.params = params;
    }
}
