use super::{GopherParseError, GopherResponse};

pub struct TextFile {
    contents: String,
}

impl GopherResponse for TextFile {
    const ITEM_TYPE: char = '0';
    fn from_str(input: &str) -> Result<Self, GopherParseError> {
        return Ok(TextFile {
            contents: input.to_string(),
        }); // TODO
    }
    fn render(&self) -> String {
        self.contents.clone()
    }
}

impl std::fmt::Display for TextFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unimplemented display for textfile")
    }
}
