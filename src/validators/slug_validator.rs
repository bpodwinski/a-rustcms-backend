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
