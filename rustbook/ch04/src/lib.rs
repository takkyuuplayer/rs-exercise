#[cfg(test)]
mod tests {
    #[test]
    fn test_4_1() {
        {
            let mut s = String::from("hello"); // use heap
            s.push_str(", world!");
            assert_eq!("hello, world!", s);
        }
        {
            // move
            let x = 5;
            let y = x;

            assert_eq!(5, x);
            assert_eq!(5, y);

            let s1 = String::from("hello");
            let s2 = s1; // move ownership to s2

            // assert_eq!("hello", s1); // compile error!
            assert_eq!("hello", s2);

            let s3 = s2.clone();
            assert_eq!("hello", s2);
            assert_eq!("hello", s3);

            takes_ownership(s2); // move ownership to takes_ownership

            // assert_eq!("hello", s2); // compile error

            makes_copy(x);
            assert_eq!(5, x);
        }
    }

    fn takes_ownership(s: String) {
        assert_eq!("hello", s);
    }

    fn makes_copy(v: i32) {
        assert_eq!(5, v);
    }
}
