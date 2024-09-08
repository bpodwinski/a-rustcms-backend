use regex::Regex;
use validator::ValidationError;

/// Validation function for slug with customizable parameters.
pub fn validate_slug(
    slug: &str,
    min_length: usize,
    max_length: usize,
) -> Result<(), ValidationError> {
    // Check minimum and maximum length
    if slug.len() < min_length || slug.len() > max_length {
        let mut error = ValidationError::new("invalid_length");
        error.message = Some(
            format!(
            "Slug length is invalid. It must be between {} and {} characters",
            min_length, max_length
        )
            .into(),
        );
        return Err(error);
    }

    // Check the format using the provided regular expression pattern
    let re = Regex::new(r"^[a-zA-Z0-9-]+$").unwrap();
    if !re.is_match(slug) {
        let mut error = ValidationError::new("invalid_format");
        error.message = Some(
            "Slug can only contain alphanumeric characters and hyphens".into(),
        );
        return Err(error);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_slug_valid() {
        let slug = "valid-slug-123";
        let result = validate_slug(slug, 5, 20);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_slug_too_short() {
        let slug = "abc";
        let result = validate_slug(slug, 5, 20);
        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(err.code, "invalid_length");
            assert_eq!(
                err.message.unwrap(),
                "Slug length is invalid. It must be between 5 and 20 characters"
            );
        }
    }

    #[test]
    fn test_validate_slug_too_long() {
        let slug = "this-is-a-very-long-slug-exceeding-the-limit";
        let result = validate_slug(slug, 5, 20);
        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(err.code, "invalid_length");
            assert_eq!(
                err.message.unwrap(),
                "Slug length is invalid. It must be between 5 and 20 characters"
            );
        }
    }

    #[test]
    fn test_validate_slug_invalid_characters() {
        let slug = "invalid_slug!";
        let result = validate_slug(slug, 5, 20);
        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(err.code, "invalid_format");
            assert_eq!(
                err.message.unwrap(),
                "Slug can only contain alphanumeric characters and hyphens"
            );
        }
    }

    #[test]
    fn test_validate_slug_contains_numbers() {
        let slug = "valid-slug-123";
        let result = validate_slug(slug, 5, 20);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_slug_contains_hyphen() {
        let slug = "valid-slug";
        let result = validate_slug(slug, 5, 20);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_slug_empty_string() {
        let slug = "";
        let result = validate_slug(slug, 1, 20);
        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(err.code, "invalid_length");
            assert_eq!(
                err.message.unwrap(),
                "Slug length is invalid. It must be between 1 and 20 characters"
            );
        }
    }
}
