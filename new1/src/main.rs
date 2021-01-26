const PI: f32 = 3.14159;

//関数の引数は`(変数名: 型名、 )`で書く
fn fizzbuzz(n: usize) {
    for i in 0..n {
        if i % 15 == 0 {
            println!("Fizzbuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn main() {
    fizzbuzz(20);

    // xの型を推論
    let x = 13;
    println!("{}", x);

    // xの型を指定
    let x: f64 = 3.14159;
    println!("{}", x);

    // 宣言の後で初期化
    let x;
    x = 0;
    println!("{}", x);

    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);

    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    println!("ゼロからアップル{}を作るには", PI);
}
