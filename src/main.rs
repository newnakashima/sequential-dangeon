use std::io::{stdin, stdout, Write};
use rand::Rng;
fn main() {
    let mut message = String::from("Hi!");
    let mut current_monster: Monster = Monster::new("", 0, 0, 0);
    let mut hero: Hero = Hero::new("勇者", 100, 10, 5);
    loop {
        game_loop(&mut message, &mut hero, &mut current_monster);
    }
}

// ゲームのメインループ
fn game_loop(message: &mut String, hero: &mut Hero, current_montser: &mut Monster) {
    // モンスターの一覧
    let monsters = monsters();
    let monster_new_generated = current_montser.hp <= 0;
    // モンスターがhpが02かどうか
    if monster_new_generated {
        // モンスターをランダムで選択して生成
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..monsters.len());
        *current_montser = monsters[n].clone();
        *message = format!("{}が現れた！", current_montser.name);
    } else {
        // モンスターが生きている場合はモンスターの情報を表示
        *message = format!("{}のHP: {}", current_montser.name, current_montser.hp);
    }

    print!("{}\n", message);

    // ユーザーの入力を促すプロンプト
    print!("どうする？\n{}: 攻撃\n{}: 防御\n{}: 逃げる\n\n{}のHP: {}\n>",
        1,
        2,
        3,
        hero.name,
        hero.hp
    );

    // ユーザーの入力を受け取る
    let mut input = String::new();
    let _ = stdout().flush();

    let result = stdin().read_line(&mut input);
    match result {
        Ok(_) => {
            match input.trim().parse::<i32>().unwrap() {
                1 => {
                    print!("{}の攻撃！\n", hero.name);
                    hero.attack_to(current_montser);
                    if current_montser.hp <= 0 {
                        println!("{}を殺した", current_montser.name);
                        return;
                    } else {
                        current_montser.attack_to(hero);
                        if hero.hp == 0 {
                            println!("{}は死んでしまった...", hero.name);
                        }
                    }
                },
                2 => {
                    hero.defense *= 2;
                },
                3 => {
                    *message = format!("{}は逃げ出した！", hero.name);
                    current_montser.hp = 0;
                },
                _ => {
                    panic!("入力エラー");
                }
            }
        },
        Err(_) => {
            panic!("入力エラー");
        }
    }

    *message = input;
}

fn monsters() -> Vec<Monster> {
    let monster_a = Monster::new("ゴブリン", 20, 5, 3);
    let monster_b = Monster::new("スライム", 15, 4, 4);

    return vec![
        monster_a,
        monster_b,
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
    fn damaged(&mut self, damage: i32);
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

    fn damaged(&mut self, damage: i32) {
        print!("{}は{}のダメージを受けた！\n", self.name, damage);
        self.hp -= damage;
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Attackable for Hero {
    fn attack<T: Attackable>(&self) -> i32 {
        self.attack
    }

    fn defense<T: Attackable>(&self) -> i32 {
        self.defense
    }

    fn damaged(&mut self, damage: i32) {
        print!("{}は{}のダメージを受けた！\n", self.name, damage);
        self.hp -= damage;
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}