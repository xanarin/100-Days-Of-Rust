// is_prime implements trial division
pub fn is_prime(candidate: i64) -> bool {
    if candidate % 2 == 0 || candidate % 3 == 0 || candidate % 5 == 0{
        return false;
    }

    // Compute sqrt(n) to determine the maximum potential factor
    let max = (candidate as f64).sqrt().ceil().round() as i64;
    let mut counter = 1;
    while (counter * 6 + 1) <= max {
        if candidate % (counter * 6 + 1) == 0 ||
            candidate % (counter * 6 + 5) == 0 {
            return false;
        }
        counter += 1;
    }
    true
}

pub fn next_prime(candidate: i64) -> i64 {
    if is_prime(candidate) {
        return candidate;
    }

    // Keep incrementing
    let mut new_num = candidate;

    // Force onto odd number, because all even numbers are automatically non-prime
    if new_num % 2 == 0 {
        new_num += 1;
    } else {
        new_num += 2;
    }

    loop {
        if is_prime(new_num) {
            return new_num;
        }
        new_num += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(next_prime(11), 11);
        assert_eq!(next_prime(12), 13);
        assert_eq!(next_prime(24), 29);
        assert_eq!(next_prime(5903), 5903);
        // Let's get some big primes up in here
        assert_eq!(next_prime(1_000_001_532), 1_000_001_537);
        assert_eq!(next_prime(1_009_904_509), 1_009_904_509);
        assert_eq!(next_prime(8_888_888_888), 8_888_888_891);
    }
}
