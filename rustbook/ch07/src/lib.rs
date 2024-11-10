pub mod back_of_house;
pub mod front_of_house; // src/front_of_house.rs // src/back_of_house.rs

#[cfg(test)]
mod tests {

    #[test]
    fn test_7_3() {
        {
            // absolute path
            assert_eq!(
                crate::front_of_house::hosting::hosting::add_to_waitlist(),
                ()
            );

            // relative path
            assert_eq!(hosting::add_to_waitlist(), ());
        }
        {
            let mut meal =
                super::back_of_house::Breakfast::summer(super::back_of_house::Toast::Rye);

            assert_eq!(meal.toast, super::back_of_house::Toast::Rye);

            meal.toast = super::back_of_house::Toast::Wheat;

            assert_eq!(meal.toast, super::back_of_house::Toast::Wheat);
        }
    }

    use crate::front_of_house::hosting::hosting;
    use crate::front_of_house::hosting::hosting as hosting2;
    use crate::front_of_house::hosting::hosting::*;

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
