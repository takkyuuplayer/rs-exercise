#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, ErrorKind, Read, Write};

    #[test]
    fn test_9_2() {
        {
            let f = File::open("hello.txt");
            let mut f = match f {
                Ok(file) => file,
                Err(ref err) if err.kind() == ErrorKind::NotFound => {
                    match File::create("hello.txt") {
                        Ok(fc) => fc,
                        Err(e) => {
                            panic!(
                                //ファイルを作成しようとしましたが、問題がありました
                                "Tried to create file but there was a problem: {:?}",
                                e
                            )
                        },
                    }
                },
                Err(err) => {
                    panic!("{:?}", err)
                }
            };
        }
        {
            let s = read_username_from_file().unwrap();
            assert_eq!(s, "");
        }
    }
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

}
