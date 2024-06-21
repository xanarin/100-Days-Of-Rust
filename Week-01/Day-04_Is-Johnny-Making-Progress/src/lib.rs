pub fn progress_days(days: Vec<u64>) -> usize {
    let mut upcount = 0;
    for i in 0..days.len() - 1 {
        if days[i + 1] > days[i] {
            upcount += 1;
        }
    }
    upcount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(progress_days(vec![3, 4, 1, 2]), 2);

        assert_eq!(progress_days(vec![10, 11, 12, 9, 10]), 3);

        assert_eq!(progress_days(vec![6, 5, 4, 3, 2, 9]), 1);

        assert_eq!(progress_days(vec![9, 9]), 0);
    }
}
