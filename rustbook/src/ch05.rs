#[cfg(test)]
mod tests {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    #[allow(dead_code)]
    struct Color(i32, i32, i32);
    #[allow(dead_code)]
    struct Point(i32, i32, i32);

    #[test]
    fn test_struct() {
        {
            // basic
            let mut user1 = User {
                username: String::from("someusername123"),
                email: String::from("someone@example.com"),
                active: true,
                sign_in_count: 1,
            };

            assert_eq!(user1.email, String::from("someone@example.com"));

            user1.email = String::from("another@example.com");

            assert_ne!(user1.email, String::from("someone@example.com"));
        }
        {
            let user = build_user(String::from("some@example.com"), String::from("some"));
            assert_eq!(user.email, String::from("some@example.com"));
        }
        {
            // updates
            let user = build_user(String::from("some@example.com"), String::from("some"));
            assert_eq!(user.email, String::from("some@example.com"));
            assert_eq!(user.username, String::from("some"));

            let user2 = User {
                email: String::from("a@example.com"),
                username: String::from("Alice"),
                ..user
            };

            assert_ne!(user.email, user2.email);

            assert_eq!(user.active, user2.active);
            assert_eq!(user.sign_in_count, user2.sign_in_count);
        }
        {
            // tuple struct
            let black = Color(0, 0, 0);
            assert_eq!(black.0, 0);

            let origin = Point(0, 0, 0);
            assert_eq!(origin.0, 0);
        }
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    #[test]
    fn struct_debug_format() {
        {
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };
            assert_eq!(1500, area(&rect1));

            assert_eq!(
                "rect1 is Rectangle { width: 30, height: 50 }",
                format!("rect1 is {rect1:?}"),
            );

            assert_eq!(
                "rect1 is Rectangle {
    width: 30,
    height: 50,
}",
                format!("rect1 is {rect1:#?}"),
            );
        }
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    #[test]
    fn struct_method() {
        {
            // method without arguments
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };
            assert_eq!(1500, rect1.area());
        }
        {
            // method with arguments
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };
            let rect2 = Rectangle {
                width: 10,
                height: 40,
            };
            let rect3 = Rectangle {
                width: 60,
                height: 45,
            };
            assert!(rect1.can_hold(&rect2));
            assert!(!rect2.can_hold(&rect3));
        }
        {
            // Associated Functions ( similar to static/module functions )
            let square = Rectangle::square(3);
            assert_eq!(3, square.width);
            assert_eq!(3, square.height);
        }
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}
