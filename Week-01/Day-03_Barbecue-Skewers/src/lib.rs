pub fn count_skewers(input: Vec<&str>) -> (usize, usize) {
    let mut veg_count = 0;

    for skewer in input.iter() {
        // Ensure that there is at least one veggie, but that no meat is present
        if skewer.chars().filter(|c| *c == 'o').count() > 0 &&
            skewer.chars().filter(|x| *x == 'x').count() == 0 {
                veg_count += 1;
        }
    }

    (veg_count, input.len() - veg_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(count_skewers(vec![
                "--xo--x--ox--",
                "--xx--x--xx--",
                "--oo--o--oo--",
                "--xx--x--ox--",
                "--xx--x--ox--"]), (1, 4));

        assert_eq!(count_skewers(vec![
                "--oooo-ooo--",
                "--xx--x--xx--",
                "--o---o--oo--",
                "--xx--x--ox--",
                "--xx--x--ox--"]), (2, 3));

        assert_eq!(count_skewers(vec![
                "--oooo-ooo--",
                "--xxxxxxxx--",
                "--o---",
                "-o-----o---x--",
                "--o---o-----"]), (3, 2));
    }
}
