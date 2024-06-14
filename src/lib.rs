mod util;

use crate::util::{to_camel_case, to_lower_case, to_snake_case, to_title_case, to_upper_case};

pub struct TransCase {
    words: Vec<String>,
}

/// Case
///
pub enum Case {
    /// Upper Case
    Upper,
    /// Lower Case
    Lower,
    /// Title Case
    Title,
    /// Snake Case
    Snake,
    /// Camel Case
    Camel,
}

/// Trans Case
///
/// Transform case.
///
///
/// ## Example
//
/// ```rs
/// use trans_case::{TransCase, Case};
///
/// let sentence = TransCase::new("trans-case in rust");
///
/// println!("{}", sentence.case(Case::Upper)); // TRANS CASE IN RUST
/// println!("{}", sentence.case(Case::Title)); // Trans Case In Rust
/// println!("{}", sentence.case(Case::Camel)); // transCaseInRust
/// ```
///
impl TransCase {
    pub fn new(value: &str) -> Self {
        let words = value
            .replace("-", " ")
            .replace("_", " ")
            .split(" ")
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        Self { words }
    }

    pub fn case(&self, case: Case) -> String {
        use Case::*;

        match case {
            Upper => to_upper_case(&self.words.join(" ")),
            Lower => to_lower_case(&self.words.join(" ")),
            Title => to_title_case(&self.words.join(" ")),
            Snake => to_snake_case(&self.words.join(" ")),
            Camel => to_camel_case(&self.words.join(" ")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trans_case() {
        let sentence = TransCase::new("Tanveer_evan mashup");

        assert_eq!(
            sentence.case(Case::Upper),
            String::from("TANVEER EVAN MASHUP")
        );
        assert_eq!(
            sentence.case(Case::Lower),
            String::from("tanveer evan mashup")
        );
        assert_eq!(
            sentence.case(Case::Title),
            String::from("Tanveer Evan Mashup")
        );
        assert_eq!(
            sentence.case(Case::Snake),
            String::from("tanveer_evan_mashup")
        );
        assert_eq!(
            sentence.case(Case::Camel),
            String::from("tanveerEvanMashup")
        );
    }
}
