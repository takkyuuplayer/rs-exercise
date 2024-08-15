#[cfg(test)]
mod tests {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    #[test]
    fn test_6_1() {
        {
            // basic
            assert_eq!(format!("{:?}", IpAddrKind::V4), "V4");
            assert_eq!(format!("{:?}", IpAddrKind::V6), "V6");
        }
        {
            // enum with value
            let home = IpAddr::V4(127, 0, 0, 1);
            assert_eq!(format!("{:?}", home), "V4(127, 0, 0, 1)");

            let loopback = IpAddr::V6(String::from("::1"));
            assert_eq!(format!("{:?}", loopback), stringify!(V6("::1")));
        }

        {
            // Option
            let x: Option<u32> = Some(2);
            assert_eq!(x.is_some(), true);
            assert_eq!(x.is_none(), false);
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[test]
    fn test_6_2() {
        {
            // match
            let cents = match Coin::Dime {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(_) => 25,
            };
            assert_eq!(cents, 10);
        }
        {
            // match with value
            let cents = match Coin::Quarter(UsState::Alaska) {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    assert_eq!(state, UsState::Alaska);
                    25
                }
            };
            assert_eq!(cents, 25);
        }
        {
            // Option
            fn plus_one(x: Option<i32>) -> Option<i32> {
                match x {
                    None => None,
                    Some(i) => Some(i + 1),
                }
            }

            assert_eq!(plus_one(Some(5)), Some(6));
            assert_eq!(plus_one(None), None);
        }
        {
            //default
            let cents = match Coin::Dime {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                _ => 9999,
            };
            assert_eq!(cents, 9999);
        }
    }
}
