// 部分的に定義された構造体
struct BagOfHolding<T> {
    // パラメータTを渡す事が可能
    item: Option<T>,
}

fn main() {
    // i32が入るバッグには何も入っていません
    // Noneからは型が決められない為型をしているする必要があります
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_none() {
        println!("バッグには何もない")
    } else {
        println!("バッグにはなにかある")
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42) };

    if i32_bag.item.is_some() {
        println!("バッグにはなにかある")
    } else {
        println!("バッグには何もない!")
    }

    // match はOptionをエレガントに分解して
    // すべてのケースが処理されることを保証できます
    match i32_bag.item {
        Some(v) => println!("バッグに {} を発見", v),
        None => println!("何も見つからなかった"),
    }
}
