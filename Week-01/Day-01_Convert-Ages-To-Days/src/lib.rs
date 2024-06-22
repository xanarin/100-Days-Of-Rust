pub fn calc_age(years: u64) -> u64 {
    years * 365
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_examples() {
        //calcAge(65) ➞ 23725
        assert_eq!(calc_age(65), 23725);

        //calcAge(0) ➞ 0
        assert_eq!(calc_age(0), 0);

        //calcAge(20) ➞ 7300
        assert_eq!(calc_age(20), 7300);
    }
}
