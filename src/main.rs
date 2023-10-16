use std::{io::{stdin, stdout, Write}, process::exit};
use rand::Rng;
use sequential_dungeon::character::{HasCharacterParameters, Monster, Hero, HasLevel};

struct GameState {
    hero: Hero,
    current_monster: Monster,
}

fn main() {
    let current_monster: Monster = Monster::new("", 0, 0, 0, 0);
    let hero: Hero = Hero::new();

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
    let monster_new_generated = game_state.current_monster.params().hp == 0;
    // モンスターがhpが0かどうか
    if monster_new_generated {
        // モンスターをランダムで選択して生成
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..monsters.len());
        game_state.current_monster = monsters[n].clone();
        println!("{}が現れた！", game_state.current_monster.params().name);
    } else {
        // モンスターが生きている場合はモンスターの情報を表示
        let monster = game_state.current_monster.params();
        println!("{}のHP: {}", monster.name, monster.hp);
        game_state.current_monster.params().set_defensing(false);

    }

    game_state.hero.params().set_defensing(false);

    let hero_params = game_state.hero.params();

    // ユーザーの入力を促すプロンプト
    print!("どうする？\n{}: 攻撃\n{}: 防御\n{}: 逃げる\n\n{}のHP: {}\n>",
        1,
        2,
        3,
        hero_params.name,
        hero_params.hp
    );

    // ユーザーの入力を受け取る
    let mut input = String::new();
    let _ = stdout().flush();

    if let Err(_) = stdin().read_line(&mut input) {
        panic!("入力エラー");
    }

    match input.trim().trim().parse::<i32>() {
        Ok(1) => {
            print!("{}の攻撃！\n", game_state.hero.params().name);
            game_state.hero.params().attack_to(&mut game_state.current_monster);
            if game_state.current_monster.params().hp == 0 {
                println!("{}を殺した", game_state.current_monster.params().name);
                return;
            } else {
                println!("{}の攻撃！", game_state.current_monster.params().name);
                game_state.current_monster.params().attack_to(&mut game_state.hero);
                if game_state.hero.params().hp == 0 {
                    println!("{}は死んだ", game_state.hero.params().name);
                    exit(0);
                }
            }
        },
        Ok(2) => {
            game_state.hero.params().set_defensing(true);
            println!("{}は防御した！", game_state.hero.params().name);
            monster_attack(&mut game_state.current_monster, &mut game_state.hero);
        },
        Ok(3) => {
            println!("負け犬{}は逃げ出した！", game_state.hero.params().name);
            game_state.current_monster.params().set_hp(0)
        },
        Ok(_) => {
            println!("コマンドミス！");
            println!("{}の攻撃！", game_state.current_monster.params().name);
            game_state.current_monster.params().attack_to(&mut game_state.hero);
            if game_state.hero.params().hp == 0 {
                println!("{}は死んだ", game_state.hero.params().name);
                exit(0);
            }
        },
        Err(_) => panic!("入力エラー"),
    }
}

fn monster_attack(monster: &mut Monster, hero: &mut Hero) {
    println!("{}の攻撃！", monster.params().name);
    monster.params().attack_to(hero);
    if hero.params().hp == 0 {
        println!("{}は死んだ", hero.params().name);
        exit(0);
    }
}

fn monsters() -> Vec<Monster> {
    let goblin = Monster::new("ゴブリン", 20, 5, 3, 1);
    let slime = Monster::new("スライム", 15, 4, 4, 1);
    let dragon = Monster::new("ドラゴン", 50, 10, 10, 5);
    let king_devil = Monster::new("魔王", 1000, 100, 20, 0);

    return vec![
        goblin,
        slime,
        dragon,
        king_devil,
    ];
}
