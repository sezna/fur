#[derive(Clone, PartialEq)]
pub enum ItemType {
    TextFile,
    GopherMap,
    Nameserver,
    Binary,
    Error,
    Image,
    MacBinary,
    Unknown,
    Information, // ... etc
}

impl ItemType {
    pub fn to_string(&self) -> String {
        match self {
            ItemType::TextFile => "TEXT",
            ItemType::GopherMap => "MENU",
            ItemType::Nameserver => "NAME",
            ItemType::Information => "INFO",
            ItemType::Binary => " BIN",
            ItemType::MacBinary => "MBIN",
            ItemType::Error => " ERR",
            ItemType::Image => "IMAG",
            ItemType::Unknown => "UNKN",
        }
        .to_string()
    }
}
