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