pub fn generate_slug(slug: &str) -> String {
    slug.trim()
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .replace(' ', "-")
}
