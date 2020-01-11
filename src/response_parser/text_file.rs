use super::{GopherParseError, GopherResponse};

pub struct TextFile;

impl GopherResponse for TextFile {
    const ItemType: char = '0';
    fn from_str(input: &str) -> Result<Self, GopherParseError> {
        return Ok(TextFile {}); // TODO
    }
    fn render(&self) -> String {
        "Unimplemented: Text File render".to_string()
    }
}

impl std::fmt::Display for TextFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unimplemented display for textfile")
    }
}
