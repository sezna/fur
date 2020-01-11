/*
struct Nameserver;
struct ErrorCode;
struct MacBinHex;
struct DOSBinHex;
// TODO etc...
*/

mod gopher_map;
mod gopher_parse_error;
mod gopher_response;
mod item_type;
mod text_file;
pub use gopher_map::GopherMap;
pub use gopher_parse_error::GopherParseError;
pub use gopher_response::GopherResponse;
pub use item_type::ItemType;
pub use text_file::TextFile;
