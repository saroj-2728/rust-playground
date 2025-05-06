mod front_of_house {
    // All items inside a module private by default, to make public prefix the item with a pub keyword
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

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::eat_at_restaurant(); // Super helps to go to the parent module of back_of_house which here is the crate root
        // Without super gives an error
    }
    
    fn cook_order() {}
    
    pub struct Breakfast {
        // Even if struct is public, its fields would still be private until pub is prefixed
        pub toast: String,      // Public field
        seasonal_fruit: String, // Private as no pub
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // A public associated fn to construct an instance of Breakfast, (reqd cause a field is private which can't be changed or set any other way)
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("apple")
            }
        }
    }

    // For enum, if enum is public, all its variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting; // Allows to use hosting directly, i.e brings hosting into the scope of eat_at_restaurant
// Also checks privacy like any other paths

mod customer {
    use crate::hosting;
    pub fn eat_at_restaurant(){
        // use only creates shortcut for the scope in which it occurs
        // here this will throw an error as the shortcut is not for this scope
        hosting::add_to_waitlist();
        // to fix either make another shortcut with use as above or use super::hosting
    }
}

pub fn eat_at_restaurant() {
    // hosting's items can be now used as
    hosting::add_to_waitlist(); 

    // Absolute path for refering an item (here fn add_to_waitlist) in the module tree
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat"); // Public field can be changed
    
    // meal.seasonal_fruit = String::from("peaches"); // Throws errors, private fields can't be accessed

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
