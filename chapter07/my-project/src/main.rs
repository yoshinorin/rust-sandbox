use std::fmt::Result;
// as でエイリアス
use std::io::Result as IoResult;

// 複数 use する
use std::{cmp::Ordering, io};
// まとめて use する
use std::collections::*;

fn main() {
    println!("Hello, world!");
}

fn function1() -> Result {

}

fn function2() -> IoResult<()> {

}
