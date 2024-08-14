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
}
