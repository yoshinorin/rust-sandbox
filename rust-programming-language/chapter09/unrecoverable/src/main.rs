fn main() {
    let v = vec![1, 2, 3];

    v[99];
}

/*
Finished dev [unoptimized + debuginfo] target(s) in 0.46s
     Running `target\debug\unrecoverable.exe`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src\main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

環境変数設定 RUST_BACKTRACE をセットして cargo run でスタックトレース出力できる

$ set RUST_BACKTRACE=1
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\unrecoverable.exe`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src\main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\unrecoverable.exe` (exit code: 101)
*/
