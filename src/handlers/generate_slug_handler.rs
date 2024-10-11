use deunicode::deunicode;

pub fn generate_slug(slug: &str) -> String {
    let normalized_slug = deunicode(slug);

    normalized_slug
        .trim()
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ' && c != '-', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug_basic() {
        let input = "Hello World";
        let expected = "hello-world";
        assert_eq!(generate_slug(input), expected);
    }

    #[test]
    fn test_generate_slug_with_special_characters() {
        let input = "Hello, World!";
        let expected = "hello-world";
        assert_eq!(generate_slug(input), expected);
    }

    #[test]
    fn test_generate_slug_with_multiple_spaces() {
        let input = "Hello     World";
        let expected = "hello-world";
        assert_eq!(generate_slug(input), expected);
    }

    #[test]
    fn test_generate_slug_with_uppercase() {
        let input = "HELLO WORLD";
        let expected = "hello-world";
        assert_eq!(generate_slug(input), expected);
    }

    #[test]
    fn test_generate_slug_with_trailing_spaces() {
        let input = "  Hello World  ";
        let expected = "hello-world";
        assert_eq!(generate_slug(input), expected);
    }

    #[test]
    fn test_generate_slug_empty_input() {
        let input = "";
        let expected = "";
        assert_eq!(generate_slug(input), expected);
    }

    #[test]
    fn test_generate_slug_already_formatted() {
        let input = "hello-world";
        let expected = "hello-world";
        assert_eq!(generate_slug(input), expected);
    }

    #[test]
    fn test_generate_slug_with_numeric_values() {
        let input = "Hello World 2023";
        let expected = "hello-world-2023";
        assert_eq!(generate_slug(input), expected);
    }
}
