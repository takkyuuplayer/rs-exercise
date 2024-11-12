#[cfg(test)]
mod tests {
    #[test]
    fn test_8_1_vector() {
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);

        let v2 = vec![1, 2];
        assert_eq!(v, v2);

        {
            // access
            assert_eq!(v[0], 1);
            #[allow(clippy::get_first)]
            let first = v.get(0);
            assert_eq!(first, Some(&v[0]));
            assert_eq!(v.get(100), None);
        }

        {
            let mut sum = 0;
            for i in &v {
                sum += i;
            }
            assert_eq!(sum, 3);
        }

        {
            for i in &mut v {
                *i += 50;
            }
            assert_eq!(v, vec!(51, 52));
        }

        #[derive(Debug, PartialEq)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }


        #[allow(clippy::useless_vec)]
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        assert_eq!(row[0], SpreadsheetCell::Int(3));
    }

    #[test]
    #[should_panic]
    fn test_8_1_out_of_lange() {
        #[allow(clippy::useless_vec)]
        let v2 = vec![1, 2];
        #[allow(clippy::unnecessary_operation)]
        v2[100];
    }

    #[test]
    fn test_8_2_string() {
        {
            let mut s = String::new();
            s.push_str("hello");

            assert_eq!(s, String::from("hello"));
            assert_eq!(s, "hello");
            assert_eq!(s, "hello".to_string());
        }
        {
            let s1 = String::from("hello, ");
            let s2 = String::from("world!");
            let s3 = s1 + &s2;
            assert_eq!(s3, "hello, world!");
        }
        {
            let s1 = String::from("hello, ");
            let s2 = String::from("world!");
            assert_eq!(format!("{}-{}", s1, s2), "hello, -world!");
        }
        {
            let hello = "こんにちは";
            assert_eq!(hello.len(), 15);
            assert_eq!(&hello[0..3], "こ");

            assert_eq!(
                hello.chars().collect::<Vec<char>>(),
                ['こ', 'ん', 'に', 'ち', 'は']
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_8_2_string_slice() {
        let hello = "こんにちは";
        let s = &hello[0..4];
        println!("{}", s);
    }

    use std::collections::HashMap;

    #[test]
    fn test_8_3_hashmap() {
        #[allow(clippy::unnecessary_to_owned)]

        {
            // basic
            let mut scores = HashMap::new();
            scores.insert(String::from("Blue"), 10);
            scores.insert(String::from("Yellow"), 50);

            assert_eq!(scores.get("Blue"), Some(&10));
            assert_eq!(scores.get("Red"), None);
        }
        {
            // zip
            #[allow(clippy::useless_vec)]
            let teams = vec![String::from("Blue"), String::from("Yellow")];
            #[allow(clippy::useless_vec)]
            let initial_scores = vec![10, 50];
            let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

            #[allow(clippy::unnecessary_to_owned)]
            let val = scores.get(&"Blue".to_string());
            assert_eq!(Some(&&10), val);
            #[allow(clippy::unnecessary_to_owned)]
            let val = scores.get(&"Red".to_string());
            assert_eq!(None, val);
        }
        {
            // re-insert
            let mut scores = HashMap::new();
            scores.insert(String::from("Blue"), 10);
            scores.insert(String::from("Blue"), 20);
            assert_eq!(scores.get("Blue"), Some(&20));

            scores.entry(String::from("Red")).or_insert(30);
            scores.entry(String::from("Red")).or_insert(40);
            assert_eq!(scores.get("Red"), Some(&30));

            let text = "hello world wonderful world";
            let mut map = HashMap::new();
            for word in text.split_whitespace() {
                let count = map.entry(word).or_insert(0);
                *count += 1;
            }
            assert_eq!(map.get(&"world"), Some(&2));
        }
    }
}
