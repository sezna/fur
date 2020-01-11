use super::{GopherParseError, GopherResponse};

pub struct TextFile;

impl GopherResponse for TextFile {
    const ItemType: char = '0';
    fn from_str(input: &str) -> Result<Self, GopherParseError> {
        return Ok(TextFile {}); // TODO
    }
}

impl std::fmt::Display for TextFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unimplemented display for textfile")
    }
}
