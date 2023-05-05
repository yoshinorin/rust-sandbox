// デバッグ用のトレイトを実装する
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 構造体に実装を持たせる
impl Rectangle {
    // データの読み込みのみで書き込みの必要なしのため、借用のみ
    // インスタンスを変更したい場合は `&mut self` とする
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数
    // 構造体に紐づけられているが `self` は引数にとらない
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!("square is {:?}", square);
}
