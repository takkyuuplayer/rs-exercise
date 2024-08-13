#[cfg(test)]
mod tests {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    #[test]
    fn test_5_1() {
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
    fn test_5_2() {
        {
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };
            assert_eq!(1500, area(&rect1));

            assert_eq!(
                "rect1 is Rectangle { width: 30, height: 50 }",
                format!("rect1 is {:?}", rect1),
            );

            assert_eq!(
                "rect1 is Rectangle {
    width: 30,
    height: 50,
}",
                format!("rect1 is {:#?}", rect1),
            );
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
