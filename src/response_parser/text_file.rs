use super::{GopherParseError, GopherResponse};
pub struct TextFile;

impl GopherResponse for TextFile {
    const ItemType: char = '0';
    fn from_str(input: &str) -> Result<Self, GopherParseError> {
        return Ok(TextFile {}); // TODO
    }
}
