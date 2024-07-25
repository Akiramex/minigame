
mod monster;

pub use crate::monster::Monster; 
use std::{io, process};
fn start() {

    println!("Welcome to minigame");
    println!("1:create_monster 2:battle 3:quit");

    loop {
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
            1 => {create_monster()},
            2 => {battle()},
            3 => {quit()},
            _ => println!("Error input"),
        }
    }
}

fn create_monster() {
    println!("create_monster");
}

fn battle() {
    println!("battle");
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
}

