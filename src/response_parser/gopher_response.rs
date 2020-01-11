use super::GopherParseError;

pub trait GopherResponse
where
    Self: std::marker::Sized,
{
    /// The item type of this gopher response. See [wiki](https://en.wikipedia.org/wiki/Gopher_(protocol)#Item_types) for details.
    const ItemType: char;
    /// Attempt to parse a string (a response from a Gopher server) into this type.
    fn from_str(input: &str) -> Result<Self, GopherParseError>;
    /// A rendering of the response.
    fn render(&self) -> String;
}
