fn main() {

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // 前述の match と同じ
    if let Some(3) = some_u8_value {
        println!("three");
        // 他のケースは無視される（パターンマッチでワイルドカートで何もしないのと同じ）
    }
}
