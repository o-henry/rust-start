// 모듈(module)은 크레이트의 코드를 그룹화해서 가독성과 재사용성을 향상하는 방법 이며 접근성을 결정한다.
mod front_of_house;

// p 148
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 절대 경로 -> 절대경로를 더 선호한다. (코드의 정의와 호출은 서로 독립적이므로)
    crate::front_of_house::hosting::add_to_waitlist();
    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
    // use 키워드
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    meal.toast = String::from("밀빵");
    println!("{} 토스트로 주세요", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
