pub fn finding_nemo(input: &str) -> String {
    let mut words = input.split(' ');
    if let Some(idx) = words.position(|x| x == "Nemo") {
        format!("I found Nemo at {}!", idx + 1)
    } else {
        "I can't find Nemo :(".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //findNemo("I am finding Nemo !") ➞ "I found Nemo at 4!"
        assert_eq!(finding_nemo("I am finding Nemo !"), "I found Nemo at 4!");

        //findNemo("Nemo is me") ➞ "I found Nemo at 1!"
        assert_eq!(finding_nemo("Nemo is me"), "I found Nemo at 1!");

        //findNemo("I Nemo am") ➞ "I found Nemo at 2!"
        assert_eq!(finding_nemo("I Nemo am"), "I found Nemo at 2!");

        assert_eq!(finding_nemo("Nobody is here"), "I can't find Nemo :(");
    }
}
