#[cfg(test)]
mod tests {
    #[test]
    fn test_10_1() {
        {
            // basic
            struct Cacher<T>
            where
                T: Fn(u32) -> u32,
            {
                calculation: T,
                value: Option<u32>,
            }
            impl<T> Cacher<T>
            where
                T: Fn(u32) -> u32,
            {
                fn new(calculation: T) -> Cacher<T> {
                    Cacher {
                        calculation,
                        value: None,
                    }
                }
                fn value(&mut self, arg: u32) -> u32 {
                    match self.value {
                        Some(v) => v,
                        None => {
                            let v = (self.calculation)(arg);
                            self.value = Some(v);
                            v
                        }
                    }
                }
            }

            let mut expensive_result = Cacher::new(|num| num + 1);

            assert_eq!(2, expensive_result.value(1));
            assert_eq!(2, expensive_result.value(2));
        }

        {
            // capture
            let x = 4;

            let equal_to_x = |z| z == x;

            assert_eq!(4, x);

            let y = 4;
            assert!(equal_to_x(y));
        }
    }

    #[test]
    fn test_10_2() {
        {
            let v1: Vec<i32> = vec![1, 2, 3];
            let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
            assert_eq!(v2, vec![2, 3, 4]);
        }

        {
            // filtering

            #[derive(PartialEq, Debug)]
            struct Shoe {
                size: u32,
                style: String,
            }

            fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
                shoes.into_iter().filter(|s| s.size == shoe_size).collect()
            }

            let shoes = vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 13,
                    style: String::from("sandal"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ];

            let in_my_size = shoes_in_my_size(shoes, 10);

            assert_eq!(
                in_my_size,
                vec![
                    Shoe {
                        size: 10,
                        style: String::from("sneaker")
                    },
                    Shoe {
                        size: 10,
                        style: String::from("boot")
                    },
                ]
            );
        }

        {
            // own iterator
            struct Counter {
                count: u32,
            }

            impl Counter {
                fn new() -> Counter {
                    Counter { count: 0 }
                }
            }

            impl Iterator for Counter {
                type Item = u32;

                fn next(&mut self) -> Option<Self::Item> {
                    self.count += 1;

                    if self.count < 6 {
                        Some(self.count)
                    } else {
                        None
                    }
                }
            }

            {
                let mut counter = Counter::new();

                assert_eq!(counter.next(), Some(1));
            }

            {
                let sum: u32 = Counter::new()
                    .zip(Counter::new().skip(1))
                    .map(|(a, b)| a * b)
                    .filter(|x| x % 3 == 0)
                    .sum();

                assert_eq!(18, sum);
            }
        }
    }
}
