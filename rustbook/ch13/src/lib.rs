#[cfg(test)]
mod tests {
    #[test]
    fn test_10_1() {
        {
            // basic
            struct Cacher<T>
            where T: Fn(u32) -> u32
            {
                calculation: T,
                value: Option<u32>,
            }
            impl<T> Cacher<T>
            where T: Fn(u32) -> u32
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
                        },
                    }
                }
            }

            let mut expensive_result = Cacher::new(|num| {
                num + 1
            });

            assert_eq!(2, expensive_result.value(1));
            assert_eq!(2, expensive_result.value(2));
        }

        {
            // capture
            let x = 4;

            let equal_to_x = |z| z == x;

            let y = 4;
            assert!(equal_to_x(y));
        }
    }
}
