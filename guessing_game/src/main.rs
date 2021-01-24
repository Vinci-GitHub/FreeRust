use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    let hoge: hoge = 10;

    println!("{}", hoge);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Start！");
    println!("Start！");
    println!("You guess: {}", guess);
    println!("2+3={}", 2 + 3);
    println!("3-1={}", 3 - 1);
    println!("3*1={}", 3 * 1);
    println!("3/1={}", 3 / 1);
    println!("3%1={}", 3 % 1);
    println!("2+3={}", 2 + 3);
    println!("3-1={}", 3 - 1);
    println!("3*1={}", 3 * 1);
    println!("3/1={}", 3 / 1);
    println!("3%1={}", 3 % 1);
    println!("End！");
    println!("End！");
    println!("End!");
    println!("End!");
}
