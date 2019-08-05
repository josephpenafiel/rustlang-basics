mod front_of_house;
pub use crate::front_of_house::hosting; // re-exporting so that it's available outside


mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

fn serve_order() {}

pub fn eat_at_restaurant(){
    //absolute path
    //crate::front_of_house::hosting::add_to_waitlist(); // now there's the use line 
    //relative path
    //front_of_house::hosting::add_to_waitlist(); // now there's the use line
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();



    //order a breakfast for summer with rye toast
    let mut breakfast = back_of_house::Breakfast::summer("rye");
    //change to wheat
    breakfast.toast = String::from("wheat");
    println!("I'd like {} toast please", breakfast.toast );

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


