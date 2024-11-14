#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(clippy::useless_vec)]
    #[test]
    fn test_10_2_trait() {
        pub trait Summary {
            fn summarize_author(&self) -> String;

            fn summarize(&self) -> String {
                // "（{}さんの文章をもっと読む）"
                format!("(Read more from {}...)", self.summarize_author())
            }
        }
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        impl Summary for NewsArticle {
            fn summarize_author(&self) -> String {
                format!("@{}", self.author)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        {
            // Basic
            let tweet = Tweet {
                username: String::from("horse_ebooks"),
                content: String::from(
                    // もちろん、ご存知かもしれませんがね、みなさん
                    "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
            };
            assert_eq!(
                "horse_ebooks: of course, as you probably already know, people",
                tweet.summarize()
            );
        }

        {
            // Default
            let article = NewsArticle {
                headline: String::from("Penguins win the Stanley Cup Championship!"),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
                content: String::from(
                    "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
                ),
            };
            assert_eq!("(Read more from @Iceburgh...)", article.summarize());
        }

        {
            let tweet = Tweet {
                username: String::from("horse_ebooks"),
                content: String::from(
                    // もちろん、ご存知かもしれませんがね、みなさん
                    "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
            };
            assert_eq!(
                notify(&tweet),
                "Breaking news! horse_ebooks: of course, as you probably already know, people"
            );
            assert_eq!(notify2(&tweet), notify(&tweet));
            assert_eq!(notify3(&tweet), notify(&tweet));
        }

        {
            assert_eq!(largest(&vec![34, 50, 25, 100, 65]), 100);
            assert_eq!(largest(&vec!['y', 'm', 'a', 'q']), 'y');
        }

        {
            // Generic な struct に メソッド実装
            use std::fmt::Display;

            struct Pair<T> {
                x: T,
                y: T,
            }

            impl<T> Pair<T> {
                fn new(x: T, y: T) -> Self {
                    Self { x, y }
                }
            }

            impl<T: Display + PartialOrd> Pair<T> {
                fn cmp_display(&self) -> String {
                    if self.x >= self.y {
                        format!("The largest member is x = {}", self.x)
                    } else {
                        format!("The largest member is y = {}", self.y)
                    }
                }
            }

            {
                let p = Pair::new(1, 2);
                assert_eq!(p.cmp_display(), "The largest member is y = 2");
            }
            {
                let p = Pair::new('b', 'a');
                assert_eq!(p.cmp_display(), "The largest member is x = b");
            }
        }

        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];

            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        pub fn notify(item: &impl Summary) -> String {
            format!("Breaking news! {}", item.summarize())
        }
        pub fn notify2<T: Summary>(item: &T) -> String {
            format!("Breaking news! {}", item.summarize())
        }
        pub fn notify3<T>(item: &T) -> String
        where
            T: Summary,
        {
            format!("Breaking news! {}", item.summarize())
        }
    }

    #[test]
    fn test_10_3_lifetime() {
        {
            fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
                if x.len() > y.len() {
                    x
                } else {
                    y
                }
            }

            let string1 = String::from("long string is long");
            {
                let string2 = String::from("xyz");
                let result = longest(string1.as_str(), string2.as_str());
                assert_eq!("long string is long", result);
            }
        }
        {
            struct ImportantExcerpt<'a> {
                part: &'a str,
            }
            let novel = String::from("Call me Ishmael. Some years ago...");
            let first_sentence = novel.split('.').next().expect("Could not find a '.'");
            let i = ImportantExcerpt {
                part: first_sentence,
            };
            assert_eq!(i.part, "Call me Ishmael")
        }
    }
}
