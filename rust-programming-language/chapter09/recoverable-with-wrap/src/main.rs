use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    // unwarp は OK なら OK の中身を、そうでなければ panic! マクロを呼び出す
    let f: File = File::open("hello.txt").unwrap();
    /*
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target\debug\recoverable-with-wrap.exe`
        thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "指定されたファイルが見つかりません。" }', src\main.rs:5:43
     */

    let f: File = File::open("hello.txt").expect("expect の場合は任意のメッセージを指定できる");
    /*
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target\debug\recoverable-with-wrap.exe`
        thread 'main' panicked at 'expect の場合は任意のメッセージを指定できる: Os { code: 2, kind: NotFound, message: "指定されたファイルが見つかりません。" }', src\main.rs:7:43
     */
}


fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
