/*
struct Nameserver;
struct ErrorCode;
struct MacBinHex;
struct DOSBinHex;
// TODO etc...
*/

mod gopher_parse_error;
mod gopher_response;
mod item_type;
mod submenu;
mod text_file;
pub use gopher_parse_error::GopherParseError;
pub use gopher_response::GopherResponse;
pub use item_type::ItemType;
pub use submenu::Submenu;
pub use text_file::TextFile;

pub fn parse(input: &str, item_type: ItemType) -> Result<Submenu, GopherParseError> {
    match item_type {
        ItemType::Submenu => Submenu::from_str(input),
        _ => Err(GopherParseError {}),
    }
}
