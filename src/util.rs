pub fn to_lower_case(value: &str) -> String {
    value
        .chars()
        .into_iter()
        .map(|c| c.to_lowercase())
        .map(|c| c.to_string())
        .collect::<String>()
}

pub fn to_upper_case(value: &str) -> String {
    value
        .chars()
        .into_iter()
        .map(|c| c.to_uppercase())
        .map(|c| c.to_string())
        .collect::<String>()
}

pub fn to_title_case(value: &str) -> String {
    let mut previous_sep = true;

    value
        .chars()
        .into_iter()
        .map(|c| {
            if "-_ ".contains(c) {
                previous_sep = true;

                ' '.to_string()
            } else if previous_sep == true {
                previous_sep = false;

                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect::<String>()
}

pub fn to_snake_case(value: &str) -> String {
    value
        .chars()
        .into_iter()
        .map(|c| if "-_ ".contains(c) { '_' } else { c })
        .map(|c| c.to_lowercase())
        .map(|c| c.to_string())
        .collect::<String>()
}

pub fn to_camel_case(value: &str) -> String {
    let mut first_letter = true;
    let mut previous_sep = false;

    value
        .chars()
        .into_iter()
        .map(|c| {
            if first_letter == true {
                first_letter = false;

                c.to_lowercase().to_string()
            } else if "-_ ".contains(c) {
                previous_sep = true;
                
                String::new()
            } else if previous_sep == true {
                previous_sep = false;

                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_lower_case() {
        assert_eq!(to_lower_case("HELLO"), String::from("hello"));
    }

    #[test]
    fn test_to_upper_case() {
        assert_eq!(to_upper_case("hello"), String::from("HELLO"));
    }

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello case"), String::from("Hello Case"));
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("Hello Case"), String::from("hello_case"));
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("Hello case"), String::from("helloCase"));
    }
}
