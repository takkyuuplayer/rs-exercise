const MAX_POINTS: u32 = 100_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_1() {
        let x = 5;

        assert_eq!(100_000, MAX_POINTS);

        // shadowing
        let x = x + 1;
        assert_eq!(6, x);
        {
            let x = x + 1;
            assert_eq!(7, x);
        }
        assert_eq!(6, x);

        let spaces = "     ";
        let spaces = spaces.len();
        assert_eq!(5, spaces);
    }

    #[test]
    fn test_3_2() {
        {
            let guess: u32 = "42".parse().expect("Not a number!");
            assert_eq!(42, guess);
        }

        {
            // integer
            assert_eq!(16, 1_6);
            assert_eq!(16, 0x10);
            assert_eq!(16, 0o20);
            assert_eq!(16, 0b10000);

            let a: u64 = 16;
            let b = 16u64;
            assert_eq!(a, b);
        }

        {
            // char
            let c = 'z';
            assert_eq!('z', c);
        }

        {
            // tuple
            let tup: (i32, f64, u8) = (500, 6.4, 1);
            let (x, y, z) = tup;

            assert_eq!(500, x);
            assert_eq!(6.4, y);
            assert_eq!(1, z);

            assert_eq!(500, tup.0);

            let tup2 = (500, 6.4, 1);
            assert_eq!(tup, tup2);
        }

        {
            // array
            let a = [1, 2, 3, 4, 5];
            let b: [i32; 5] = [1, 2, 3, 4, 5];

            assert_eq!(a, b);

            let c = [3; 5];
            assert_eq!([3,3,3,3,3], c);
        }
    }

    #[test]
    #[should_panic]
    fn test_3_2_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        let idx: usize = "5".parse().expect("must parse");
        print!("{}", a[idx]);
    }
}
