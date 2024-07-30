mod utils;
mod monster;

pub use crate::monster::Monster; 
use std::{collections::VecDeque, io, process, thread, time::Duration};
use monster::State;
use rand::Rng;
use colored::*;
fn start() {

    let mut has_monster = VecDeque::<Monster>::new();

    println!("{}", "Welcome to minigame".blue());


    loop {

        show_choose();

        let mut buffer = String::new();

        io::stdin().read_line(&mut buffer).expect("Failed to read line");
    
        let choise : i32 = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error input");
                continue
            },
        };

        match choise {
            1 => {
                println!("{}", "--------------------------".purple());
                has_monster.push_back(create_monster());
            },
            2 => {
                println!("{}", "--------------------------".purple());
                go_outside(&mut has_monster);
            },
            3 => {
                println!("{}", "--------------------------".purple());
                show_monster(&has_monster);
            },
            0 => {
                println!("{}", "--------------------------".purple());
                quit();
                println!("{}", "--------------------------".purple());
            }
            _ => println!("Error input"),
        }
    }
}

fn create_monster() -> Monster {
    println!("创建Monster");
    println!("给你的Monster取个名字: ");

    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Failed to read line");

    let rand_hp = rand::thread_rng().gen_range(1..=100);
    let rand_mp = rand::thread_rng().gen_range(1..=100);
    let rand_attack = rand::thread_rng().gen_range(1..=10);

    let new_monster = Monster::build(name.trim().to_string(), rand_hp, rand_mp, rand_attack);

    println!("{}", "创建成功:".green());
    println!("{new_monster}");
    new_monster
}

fn show_monster(list: &VecDeque<Monster>) {
    for (i, item) in list.iter().enumerate() {
        println!("No.{i} monster");
        println!("{item}\n")
    }
}
fn show_choose() {
    println!("{}", "--------------------------".purple());
    println!("1:创建Monster");
    println!("2:到外边走走");
    println!("3:看看手上的绳子");
    println!("{}", "0:退出".red());
    println!("{}", "--------------------------".purple());
}

fn show_battlle() {
    println!("输入指令 -> 1:普通攻击 2:重击 3:啥也不干");
}

fn get_random_monster(begin :u32, end :u32) -> Result<Monster, &'static str>{
    
    if end < begin {
        return Err("input Error");
    }

    let rand_attack_end = end >> 2;

    let base_name= vec!["Yoto", "Leo", "Akira", "Mila", "Santo", "Alan"];

    let rand_name_index = rand::thread_rng().gen_range(0..base_name.len());
    let rand_hp  = rand::thread_rng().gen_range(begin..=end) as i32;
    let rand_mp = rand::thread_rng().gen_range(begin..=end) as i32;
    let rand_attack = rand::thread_rng().gen_range(0..=rand_attack_end) as i32;

    let rand_name = base_name[rand_name_index];

  
    Ok(Monster::build(rand_name.to_string(), rand_hp, rand_mp, rand_attack))
}

fn go_outside(list: &mut VecDeque<Monster>) {

    if list.is_empty() {
        println!("你还没有Monster呢，去创建你的Monster吧!");
        return;
    }

    println!("{}", "一脚踢飞路边的野草!!".green());

    let mut rand_monster  =  get_random_monster(1, 50).unwrap();
    thread::sleep(Duration::from_secs(1));
    println!("{}{}", "遭遇了野生的Monster: ".red(), rand_monster.name());
    


    'bigloop: loop {

        let monster_index;

        // 每次循环检测一下monster的状态
        let mut alive_monster_num = 0;
        for monster in list.iter() {
            if let State::Live = monster.state() {
                alive_monster_num += 1;
            }
        }

        if alive_monster_num == 0 {
            println!("{}", "你的Monster全睡着了".red());
            println!("{}", "GAME OVER".red());
            break;
        }
        else {
            println!("选择你出战的Monster：");
            show_monster(list);

            loop {
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).expect("Failed to read line");
                let choise : usize = match buffer.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Error input");
                        continue
                    },
                };

                if let Some(m) = list.get_mut(choise) {
                    match m.state() {
                        State::Live => {
                            monster_index = choise;
                            break;
                        }
                        State::Sleep => {
                            println!("{}睡着了，不能塔塔开", m.name());
                            continue
                        },
                    };
                } else {
                    continue;
                }
            }
        }


        loop {
            // 选择战斗指令
            // 我方睡着break
            // 敌方睡着直接break 'bigloop;

            show_battlle();
            println!("{}", "--------------------------".purple());
    
            let mut buffer = String::new();
        
            io::stdin().read_line(&mut buffer).expect("Failed to read line");
    
            let choise : i32 = match buffer.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Error input");
                    continue
                },
            };
    
            match choise {
                1 => {
                    if let Some(mst) = list.get_mut(monster_index) {
                        let (a, h) = mst.normal_attack(&mut rand_monster);
                        println!("你的Monster:{}，使用了普通攻击，对{}造成了{}点伤害， {}剩下{}血", mst.name(), rand_monster.name(), a, rand_monster.name(), h);
                        thread::sleep(Duration::from_secs(1));
                    }
                },
                2 => {
                    if let Some(mst) = list.get_mut(monster_index) {
                        println!("你的Monster:{}，使用了重击，但是摔倒了，Miss~", mst.name());
                        thread::sleep(Duration::from_secs(1));
                    }
                },
                3 => {
                    if let Some(mst) = list.get_mut(monster_index) {
                        println!("你的Monster:{} 在发呆~", mst.name());
                        thread::sleep(Duration::from_secs(1));
                    }
                },
                _ => (),
            }
            
    
            if let State::Sleep = rand_monster.state() {
                println!("登登登，战胜了{}，获得了xp", rand_monster.name());

                println!("{}", "YOU WIN".red());
                thread::sleep(Duration::from_secs(2));
                break 'bigloop;
            }
            
                
            let rand_index  = rand::thread_rng().gen_range(1..=10);
            // 这里搞怪物的逻辑
            match rand_index {
                1..=6 => {
                    if let Some(mst) = list.get_mut(monster_index) {
                        let (a, h) = rand_monster.normal_attack(mst);
                        println!("Monster:{}，使用了普通攻击，对{}造成了{}点伤害， {}剩下{}血", rand_monster.name(), mst.name(), a, mst.name(), h);
                        thread::sleep(Duration::from_secs(1));
                    }
                },
                7|8 => {
                    if let Some(mst) = list.get_mut(monster_index) {
                        let (a, h) = rand_monster.power_attack(mst);
                        println!("Monster:{}，使用了重击，对{}造成了{}点伤害， {}剩下{}血", rand_monster.name(), mst.name(), a, mst.name(), h);
                        thread::sleep(Duration::from_secs(1));
                    }
                },
                9|10 => {
                    println!("Monster:{}正在观察", rand_monster.name());
                    thread::sleep(Duration::from_secs(1));
                },
                _ => (),
            }
    
            if let Some(mst) = list.get(monster_index) {
                if let State::Sleep = mst.state() {
                    println!("你的Monster:{},由于血量低于0, 睡着了~", mst.name());
                    thread::sleep(Duration::from_secs(2));
                    break;
                }
            }
        }
    }
}

fn quit() {
    println!("quit");
    process::exit(0)
}
pub fn run() {
    // 1.开始界面
    // 2.等待用户输入
    // 3.根据用户输入跳转分支 -> 1.创建monster 2.开始对战 3.退出游戏
    start()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_test() {
        let a = Monster::new();

        println!("{a}")
    }

    #[test]
    fn create_test() {
        create_monster();
    }

    #[test]
    fn show_test() {
        let mut has_monster = VecDeque::<Monster>::new();

        has_monster.push_back(Monster::new());
        has_monster.push_back(Monster::new());

        show_monster(&has_monster);

        if let Some(x) = has_monster.get(0) {
            println!("{x}")
        };
    }

    #[test]
    fn one_battle() {
        let mut m1 = Monster::new();

        let rand_hp = rand::thread_rng().gen_range(1..=100);
        let rand_mp = rand::thread_rng().gen_range(1..=100);
        let rand_attack = rand::thread_rng().gen_range(1..=10);
    
        let mut m2 = Monster::build("my_monster".to_string(), rand_hp, rand_mp, rand_attack);
        
        println!("{m1}");
        println!("{m2}");

        while m1.hp() > 0 && m2.hp() > 0 {
            let (_, hp) = m1.normal_attack(&mut m2);
            println!("Monster m1 use normal_attack, m2.hp={hp}");
            if hp <= 0 {
                break;
            }

            let (_, hp) = m2.normal_attack(&mut m1);
            println!("Monster m2 use normal_attack, m1.hp={hp}");
            if hp <= 0 {
                break;
            }
        }

    }

    #[test]
    fn rand_monster() {
        match get_random_monster(233, 200) {
            Ok(x) => println!("{x}"),
            Err(e) => println!("Error: {}", e),
        }
    }
}