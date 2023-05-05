fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// 引数に参照をとることを「借用」という
fn calculate_length(s: &String) -> usize {
    // s.push_str(", world"); // 参照の場合でも不変なので、変更しようとするとエラーとなる
    s.len()
}

/*
  - ダングリングポインタ: 無効なメモリ領域を指すポインタ

fn dangle() -> &String {
    let s = String::from("hello");
    &s  // String への参照を返す.
        // dangle のコードが終了した時点で s はスコープを抜けるが、それを参照で返そうとするとエラーとなる.
}
*/

// 参照でない値を返す場合はダングリングポインタとならない
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // String を返す.
}


