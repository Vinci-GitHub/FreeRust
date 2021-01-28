struct SeaCreature {
    // Stringは構造体である
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    wepon: String,
}

fn main() {
    // staticメソッドでStringインスタンスを作成する
    let s = String::from("Hello World!");
    // インスタンスを使ってメソッドを呼び出す。
    println!("{} is {}", s, s.len());
}
