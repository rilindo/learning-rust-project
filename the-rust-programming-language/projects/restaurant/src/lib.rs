mod front_of_house;

pub use crate::front_of_house::hosting;


mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,

    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // fn fix_incorrect_order() {
    //     cook_order()
    //     super::serve_order();
    // }
 }

pub fn eat_at_restaurant() {

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();


    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("Blueberries");

    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}
