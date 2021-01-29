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

    // SeaCreatureのデータはスタックに入ります
    let ferris = SeaCreature {
        // String構造体もスタックに入りますが、
        // ヒープに入るデータの参照アドレスが一つ入ります。
        animal_type: String::from("crab"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        wepon: String::from("claw"),
    };

    let sarah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        wepon: String::from("none"),
    };

    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} wepon",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.wepon
    );
    println!(
        "{} is a {}. They have {} arms . and {} legs. They have no wepon..",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs
    );
}
