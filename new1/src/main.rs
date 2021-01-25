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
}
