// モジュールのサンプルなので実装無し
// デフォルトのスコープはプライベート
mod front_of_house {

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


