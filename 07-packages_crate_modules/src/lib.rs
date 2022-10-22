#![allow(unused)]

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn server_order() {}

//         fn take_payment() {}
//     }
// }

mod front_of_house;
pub use crate::front_of_house::hosting;

mod back_of_house;
pub use crate::back_of_house::back_house::Appetizer;
pub use crate::back_of_house::back_house::Breakfast;
pub use crate::back_of_house::back_house;

// use crate::front_of_house::hosting;
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
    let order3 = back_house::Appetizer::Soup;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

