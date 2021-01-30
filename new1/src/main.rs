#![allow(dead_code)] // この行でコンパイラのwarings messageを止めます

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}
enum PoisonType {
    Acidic,
    Painful,
    Lethal,
}
enum Size {
    Big,
    Small,
}
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None,
}

struct SeaCreature {
    sepecies: Species,
    name: String,
    arms: i32,
    legs: i32,
    wepon: Weapon,
}

fn main() {
    // SeaCreatureのデータもスタックに入ります
    let ferris = SeaCreature {
        sepecies: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        wepon: Weapon::Claw(2, Size::Small),
    };

    match ferris.sepecies {
        Species::Crab => match ferris.wepon {
            Weapon::Claw(num_claws, size) => {
                let size_description = match size {
                    Size::Big => "big",
                    Size::Small => "small",
                };
                println!(
                    "ferris is a crab with {} {} claws",
                    num_claws, size_description
                )
            }
            _ => println!("ferris is a crab with some other wepon"),
        },
        _ => println!("ferris is some other animal"),
    }
}
