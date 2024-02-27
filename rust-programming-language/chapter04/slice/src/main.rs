fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    println!("{}", word);

    s.clear();
}

/*
  - slice を使用しないバージョン

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
*/

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
            // 先頭を指定しない場合は 0 から始まるとみなされる（末尾も同様の扱い）
            // return &s[..i];
        }
    }

    // 範囲指定しなかった場合と同じ（つまり全体が返る）
    &s[..]
}
