enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v3 = Vec::new();
    v3.push(3);
    v3.push(4);
    v3.push(5);

    // `vec!` 与えられた値を保持して新しいベクタを生成するマクロ
    let v2 = vec![1,2,3,4,5];

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(x) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    /*
    &v2[100];       // panic になる
    v2.get(100);    // None が返ってくる
    */

    for i in &v2 {
        println!("{}", i);
    }

    // enum を使えば異なる型を vector にセットできる
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
