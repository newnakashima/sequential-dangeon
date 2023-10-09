use std::cmp;

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
