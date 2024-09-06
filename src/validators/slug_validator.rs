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
        return Err(ValidationError::new(
            "Slug length is invalid. It must be between the specified range",
        ));
    }

    // Check the format using the provided regular expression pattern
    let re = Regex::new(r"^[a-zA-Z0-9-]+$").unwrap();
    if !re.is_match(slug) {
        return Err(ValidationError::new(
            "Slug can only contain alphanumeric characters and hyphens",
        ));
    }

    Ok(())
}
