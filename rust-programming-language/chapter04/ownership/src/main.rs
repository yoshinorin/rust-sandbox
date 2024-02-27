fn main() {
    // メモリ確保
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{}", s);

    // s はこの後（スコープから抜けた際に）メモリ解放される。
    // Rust では所有権をもとにして自動で解放される。
    // メモリ解放には drop という特殊な関数が呼ばれる
}


fn memory_example() {
    let s1 = String::from("hello");
    // この時にムーブ（シャロ―コピー + s1 の無効化のようなもの）となる
    let s2 = s1;

    // 従ってこれはエラーになる（s1 は既に無効化されているため）
    // println!("{}, world!", s1);
}

fn deep_copy_example() {
    let s1 = String::from("hello");
    // ディープコピーしたいときは clone つかう
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
