/// Use Deserialize to convert e.g. from request JSON into Book struct.
use serde::Deserialize;
/// Demo book structure with some example fields for id, title, author.
#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Cars {
    pub id: u32,
    pub name: String,
    pub brand: String,
}

/// Display the book using the format "{title} by {author}".
/// This is a typical Rust trait and is not axum-specific.
impl std::fmt::Display for Cars {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} by {}", self.name, self.brand)
    }
}