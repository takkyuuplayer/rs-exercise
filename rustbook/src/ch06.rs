#[cfg(test)]
mod tests {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    enum IpAddr {
        #[allow(dead_code)]
        V4(u8, u8, u8, u8),
        #[allow(dead_code)]
        V6(String),
    }

    #[test]
    fn test_enum() {
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
            assert!(x.is_some());
            // assert!(!x.is_none()); // clipply に怒られる
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    enum UsState {
        #[allow(dead_code)]
        Alabama,
        Alaska,
    }

    enum Coin {
        #[allow(dead_code)]
        Penny,
        #[allow(dead_code)]
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[test]
    fn enum_match() {
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
            #[allow(clippy::manual_map)]
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

    #[test]
    fn enum_if_let() {
        let some_u8_value = Some(0u8);
        if let Some(3) = some_u8_value {
            unreachable!("this should not happen");
        } else {
            assert_eq!(some_u8_value, Some(0));
        }
    }
}
