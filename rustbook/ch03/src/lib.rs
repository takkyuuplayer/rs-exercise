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
}
