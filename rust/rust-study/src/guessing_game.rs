
extern crate rand;

use std::io;
use std::cmp::Ordering;
use self::rand::Rng;


pub fn run(){
    println!("猜数字游戏！");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("请输入你的答案：");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("读取答案失败！");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你的答案: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("恭喜你，猜对了！");
                break;
            }
        }
    }
}
