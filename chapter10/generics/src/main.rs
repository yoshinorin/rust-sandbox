/*
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("最大値は {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("最大値は {}", result);
}
*/

// 上のコードをジェネリクスを使った場合
/*
// PartialOrd trait を指定しない場合

error[E0369]: binary operation `>` cannot be applied to type `T`
  --> src\main.rs:40:17
   |
40 |         if item > largest {
   |            ---- ^ ------- T
   |            |
   |            T
   |
help: consider restricting type parameter `T`
   |
36 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
   |             ++++++++++++++++++++++


error[E0508]: cannot move out of type `[T]`, a non-copy slice
  --> src\main.rs:56:23
   |
56 |     let mut largest = list[0];
   |                       ^^^^^^^
   |                       |
   |                       cannot move out of here
   |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
   |
help: consider borrowing here
   |
56 |     let mut largest = &list[0];
   |                       +

// Copy trait を指定しない場合
// https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html#%E3%82%B9%E3%82%BF%E3%83%83%E3%82%AF%E3%81%AE%E3%81%BF%E3%81%AE%E3%83%87%E3%83%BC%E3%82%BF-%E3%82%B3%E3%83%94%E3%83%BC
error[E0507]: cannot move out of a shared reference
  --> src\main.rs:58:18
   |
58 |     for &item in list.iter() {
   |          ----    ^^^^^^^^^^^
   |          |
   |          data moved here
   |          move occurs because `item` has type `T`, which does not implement the `Copy` trait
   |
help: consider removing the borrow
   |
58 -     for &item in list.iter() {
58 +     for item in list.iter() {
   |

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
*/
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大値は {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大値は {}", result);
}
