// デバッグ用のトレイトを実装する
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // 括弧内に :? を指定すると Debug 出力整形をするという指示となる
    // Debug トレイトは構造体に対して指定しておく必要がある（1行目）
    println!("rect1 is {:?}", rect1);
    // 出力フォーマットが変わる
    println!("rect1 is {:#?}", rect1);
}

// 構造体の所有権を奪うのではなく借用する
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
