mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_atable() {}
    }

    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    #[derive(Debug, PartialEq, Eq)]
    pub enum Toast {
        Rye,
        Wheat,
    }
    pub struct Breakfast {
        pub toast: Toast,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: Toast) -> Breakfast {
            Breakfast {
                toast: toast,
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }
    fn cook_order() {}
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_7_3() {
        {
            // absolute path
            assert_eq!(crate::front_of_house::hosting::add_to_waitlist(), ());

            // relative path
            assert_eq!(super::front_of_house::hosting::add_to_waitlist(), ());
        }
        {
            let mut meal =
                super::back_of_house::Breakfast::summer(super::back_of_house::Toast::Rye);

            assert_eq!(meal.toast, super::back_of_house::Toast::Rye);

            meal.toast = super::back_of_house::Toast::Wheat;

            assert_eq!(meal.toast, super::back_of_house::Toast::Wheat);
        }
    }

    use crate::front_of_house::hosting;
    use crate::front_of_house::hosting as hosting2;
    use crate::front_of_house::hosting::*;

    use crate::back_of_house::{Breakfast, Toast};

    #[test]
    fn test_7_4() {
        {
            // use
            assert_eq!(hosting::add_to_waitlist(), ());
            assert_eq!(hosting2::add_to_waitlist(), ());
            assert_eq!(add_to_waitlist(), ());
        }

        {
            let mut meal = Breakfast::summer(Toast::Rye);

            assert_eq!(meal.toast, Toast::Rye);

            meal.toast = Toast::Wheat;

            assert_eq!(meal.toast, Toast::Wheat);
        }
    }
}
