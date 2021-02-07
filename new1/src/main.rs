fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

// mainは値は返しませんがエラーを返す事があります
fn main() -> Result<(), String> {
    let result = do_something_that_might_fail(12);

    match result {
        Ok(v) => println!("発見 {}", v),
        Err(_e) => {
            //エラーをうまく処理
            //何が起きたのかを説明する新しいerr を mainから返します
            return Err(String::from("mainでなにか問題が起きました"));
        }
    }

    // Resultのokの中にあるunit値によって
    // すべてが正常である事を表現している事に注意してください
    Ok(())
}

//コメント追加
pub mod hosting {
    pub fn add_to_waitlist() {}
}
