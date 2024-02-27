fn main() {
    let guess: u32 = "42".parse().expect("Not a number");

    // タプル
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;    // tuple の1つ目にアクセス
    let six_point_four = tup.1;
    let one = tup.2;

    println!("{}", five_hundred);

    let array = [1, 2, 3, 4, 5];
    // 要素数と型を指定する
    let array2: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{}", array[0]);
}
