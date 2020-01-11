struct GopherSubmenu;
/*
struct Nameserver;
struct ErrorCode;
struct MacBinHex;
struct DOSBinHex;
// TODO etc...
*/

mod gopher_parse_error;
mod gopher_response;
mod text_file;
pub use gopher_parse_error::GopherParseError;
pub use gopher_response::GopherResponse;
pub use text_file::TextFile;

pub fn parse(input: &str) -> Result<impl GopherResponse, GopherParseError> {
    return TextFile::from_str(" ");
}
