use std::collections::HashMap;

pub fn sock_pairs(input: &str) -> usize {
    let mut map = HashMap::new();
    let mut pairs = 0;

    // Create map of all letters to how many instances of them exist
    for letter in input.chars() {
        println!("{}: found", letter);
        map.entry(letter)
           .and_modify(|v| { *v += 1 })
           .or_insert(1);
    }

    // Count all pairs
    for count in map.values() {
        pairs += count / 2;
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sock_pairs(""), 0);

        assert_eq!(sock_pairs("AA"), 1);

        assert_eq!(sock_pairs("ABABC"), 2);

        assert_eq!(sock_pairs("CABBACCC"), 4);

        assert_eq!(sock_pairs("DDDDDDDDD"), 4);
    }
}
