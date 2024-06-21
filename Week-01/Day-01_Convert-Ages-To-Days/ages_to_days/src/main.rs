use std::env;
use anyhow::{Context, Result};


fn calc_age(years: u64) -> u64 {
    years * 365
}



fn main() -> Result<()> {
    if env::args().len() != 2 {
        eprintln!("Invalid number of arguments {}, expected {}", env::args().len(), 2)
    }
    let args: Vec<String> = env::args().collect();
    let in_years = str::parse(&args[1]).context("Failed to parse number of days")?;
    let out_days = calc_age(in_years);

    println!("Number of years ({}) -> {} days", in_years, out_days);
    Ok(())
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
