// モジュールのサンプルなので実装無し
// デフォルトのスコープはプライベート
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {

    // モジュール内に構造体を定義
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // モジュール内に enum を定義
    pub enum Appetizer {
        Soup,
        Salada,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // `back_of_house` の親モジュール つまり crate に属する `serve_order` を呼び出す
        super::serve_order();
    }

    fn cook_order() {}
}

// use で使用するクレートをスコープに持ち込む
use crate::front_of_house::hosting;

// pub で再公開できる
// as はエイリアス（エイリアスを使用しなくても再公開可）
pub use crate::front_of_house::hosting as hst;

pub fn eat_at_restaurant() {
    // 絶対指定
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対指定
    front_of_house::hosting::add_to_waitlist();

    // use で指定しているため hosting から使える
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();


    // モジュール内で定義した 構造体 を使う
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // モジュール内で定義した enum を使う
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salada;
}
