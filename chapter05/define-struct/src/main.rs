struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 名前付きフィールドのないタプル
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("hoge@example.com"),
        username: String::from("hoge"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    let mut user2 = build_user(String::from("fuga@example.com"), String::from("fuga"));
    user2.email = String::from("fuga2@example.com");

    // user1 のインスタンスから一部コピーする
    let user3 = User {
        email: String::from("piyo@example.com"),
        username: String::from("piyo"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user3.email);

    // user1 のインスタンスから一部コピーする
    let user4 = User {
        email: String::from("fuge@example.com"),
        username: String::from("fuge"),
        ..user1   // さらに省略できる
    };
    println!("{}", user4.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


fn build_user(email: String, username: String) -> User {
    User {
        email,      // 変数と構造体のフィールド名が同じ場合省略できる
        username,
        active: true,
        sign_in_count: 1,
    }
}
