#[allow(dead_code)]
const MAX_POINTS: u32 = 100_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant() {
        assert_eq!(100000, MAX_POINTS);
    }

    #[test]
    fn variables_and_mutability() {
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
    fn data_types() {
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
            assert_eq!([3, 3, 3, 3, 3], c);
        }
    }

    #[test]
    #[should_panic]
    fn out_of_array() {
        let a = [1, 2, 3, 4, 5];
        let idx: usize = "5".parse().expect("must parse");
        print!("{}", a[idx]);
    }

    #[test]
    fn statement_and_equation() {
        {
            // format
            assert_eq!("The measurement is: 5h", print_labeled_measurement(5, 'h'));
        }
        {
            // equation
            let y = {
                let x = 1;
                x + 1
            };
            assert_eq!(2, y);
        }

        {
            let x = plus_one(5);
            assert_eq!(6, x);
        }
    }

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    fn print_labeled_measurement(value: i32, unit_label: char) -> String {
        format!("The measurement is: {value}{unit_label}")
    }

    #[test]
    fn control_flow() {
        {
            #[allow(clippy::assertions_on_constants)]
            // if-else
            if true {
                assert!(true);
            } else {
                assert!(false);
            }

            let number = if true { 5 } else { 6 };
            assert_eq!(5, number);
        }

        {
            // loop
            {
                let mut count = 0;
                'label: loop {
                    let mut remaining = 10;
                    loop {
                        if remaining == 9 {
                            break;
                        }
                        if count == 2 {
                            break 'label;
                        }
                        remaining -= 1;
                    }
                    assert_eq!(9, remaining);

                    count += 1;
                }
                assert_eq!(2, count);
            }

            {
                let mut sum = 0;
                for element in [10, 20, 30] {
                    sum += element;
                }
                assert_eq!(60, sum);

                let mut sum2 = 0;
                for element in 1..5 {
                    sum2 += element;
                }
                assert_eq!(10, sum2);

                for element in (1..5).rev() {
                    sum2 -= element;
                }
                assert_eq!(0, sum2);
            }
        }
    }
}
