use std::{io::{stdin, stdout, Write}, process::exit};
use std::cmp;
use rand::Rng;

struct GameState {
    hero: Hero,
    current_monster: Monster,
}

fn main() {
    let current_monster: Monster = Monster::new("", 0, 0, 0);
    let hero: Hero = Hero::new("勇者", 100, 10, 5);

    let mut game_state = GameState {
        hero,
        current_monster,
    };

    loop {
        game_loop(&mut game_state);
    }
}

// ゲームのメインループ
fn game_loop(game_state: &mut GameState) {
    // モンスターの一覧
    let monsters = monsters();
    let monster_new_generated = game_state.current_monster.hp == 0;
    // モンスターがhpが0かどうか
    if monster_new_generated {
        // モンスターをランダムで選択して生成
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..monsters.len());
        game_state.current_monster = monsters[n].clone();
        println!("{}が現れた！", game_state.current_monster.name);
    } else {
        // モンスターが生きている場合はモンスターの情報を表示
        println!("{}のHP: {}", game_state.current_monster.name, game_state.current_monster.hp);
    }

    // ユーザーの入力を促すプロンプト
    print!("どうする？\n{}: 攻撃\n{}: 防御\n{}: 逃げる\n\n{}のHP: {}\n>",
        1,
        2,
        3,
        game_state.hero.name,
        game_state.hero.hp
    );

    // ユーザーの入力を受け取る
    let mut input = String::new();
    let _ = stdout().flush();

    let result = stdin().read_line(&mut input);
    match result {
        Ok(_) => {
            match input.trim().parse::<i32>().unwrap() {
                1 => {
                    print!("{}の攻撃！\n", game_state.hero.name);
                    game_state.hero.attack_to(&mut game_state.current_monster);
                    if game_state.current_monster.hp == 0 {
                        println!("{}を殺した", game_state.current_monster.name);
                        return;
                    } else {
                        println!("{}の攻撃！", game_state.current_monster.name);
                        game_state.current_monster.attack_to(&mut game_state.hero);
                        if game_state.hero.hp == 0 {
                            println!("{}は死んだ", game_state.hero.name);
                            exit(0);
                        }
                    }
                },
                2 => {
                    game_state.hero.defense *= 2;
                },
                3 => {
                    println!("負け犬{}は逃げ出した！", game_state.hero.name);
                    game_state.current_monster.hp = 0;
                },
                _ => {
                    println!("コマンドミス！");
                    println!("{}の攻撃！", game_state.current_monster.name);
                    game_state.current_monster.attack_to(&mut game_state.hero);
                    if game_state.hero.hp == 0 {
                        println!("{}は死んだ", game_state.hero.name);
                        exit(0);
                    }
                }
            }
        },
        Err(_) => {
            panic!("入力エラー");
        }
    }
}

fn monsters() -> Vec<Monster> {
    let goblin = Monster::new("ゴブリン", 20, 5, 3);
    let slime = Monster::new("スライム", 15, 4, 4);
    let dragon = Monster::new("ドラゴン", 50, 10, 10);
    let king_devil = Monster::new("魔王", 1000, 100, 20);

    return vec![
        goblin,
        slime,
        dragon,
        king_devil,
    ];
}

#[derive(Clone, Debug)]
struct Monster {
    name: String,
    hp: i32,
    attack: i32,
    defense: i32,
}

#[derive(Clone, Debug)]
struct Hero {
    name: String,
    hp: i32,
    attack: i32,
    defense: i32,
}

trait Attackable {
    fn name(&self) -> String;
    fn attack<T: Attackable>(&self) -> i32;
    fn defense<T: Attackable>(&self) -> i32;
    fn attack_to<T: Attackable>(&self, target: &mut T) {
        let random_range = rand::thread_rng().gen_range(-5..5);
        let damage = self.attack::<T>() - target.defense::<T>() + random_range;
        if damage > 0 {
            target.damaged(damage);
        } else {
            println!("{}はダメージを受けていない！", target.name());
        }
    }
    fn damaged(&mut self, damage: i32) {
        print!("{}は{}のダメージを受けた！\n", self.name(), damage);
        self.set_hp(cmp::max(self.get_hp() - damage, 0));
    }

    fn set_hp(&mut self, hp: i32);
    fn get_hp(&self) -> i32;
}

impl Monster {
    fn new(name: &str, hp: i32, attack: i32, defense: i32) -> Monster {
        Monster {
            name: name.to_string(),
            hp,
            attack,
            defense,
        }
    }
}

impl Hero {
    fn new(name: &str, hp: i32, attack: i32, defense: i32) -> Hero {
        Hero {
            name: name.to_string(),
            hp,
            attack,
            defense,
        }
    }
}

impl Attackable for Monster {
    fn attack<T: Attackable>(&self) -> i32 {
        self.attack
    }

    fn defense<T: Attackable>(&self) -> i32 {
        self.defense
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_hp(&self) -> i32 {
        self.hp
    }
    fn set_hp(&mut self, hp: i32) {
        self.hp = hp;
    }
}

impl Attackable for Hero {
    fn attack<T: Attackable>(&self) -> i32 {
        self.attack
    }

    fn defense<T: Attackable>(&self) -> i32 {
        self.defense
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_hp(&self) -> i32 {
        self.hp
    }
    fn set_hp(&mut self, hp: i32) {
        self.hp = hp;
    }
}