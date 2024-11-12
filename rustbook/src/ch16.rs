#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_16_1_thread() {
        {
            let handle = thread::spawn(|| {
                for i in 1..10 {
                    assert!(i > 0);
                }
            });
            handle.join().unwrap()
        }
        {
            // move
            let v = vec![1, 2, 3];

            let handle = thread::spawn(move || {
                assert_eq!(vec![1, 2, 3], v);
            });

            handle.join().unwrap();
        }
    }

    #[test]
    fn test_16_2_channel() {
        {
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let val = String::from("hi");
                tx.send(val).unwrap();

                // assert_eq!("hi", val) compile error
            });

            let received = rx.recv().unwrap();
            assert_eq!("hi", received);
        }
        {
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_millis(10));
                }
            });

            let mut receives: Vec<String> = Vec::new();
            for received in rx {
                receives.push(received);
            }
            assert_eq!(vec!["hi", "from", "the", "thread"], receives);
        }
        {
            let (tx, rx) = mpsc::channel();

            let tx1 = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                let vals = vec![2, 4, 6];

                for val in vals {
                    tx1.send(val).unwrap();
                    thread::sleep(Duration::from_millis(10));
                }
            });

            thread::spawn(move || {
                let vals = vec![1, 3, 5];

                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_millis(10));
                }
            });

            let mut receives: Vec<i32> = Vec::new();
            for received in rx {
                receives.push(received);
            }
            assert_eq!(21, receives.into_iter().sum())
        }
    }
}
