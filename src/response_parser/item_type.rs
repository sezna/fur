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
    HTML,
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
            ItemType::HTML => "HTML",
        }
        .to_string()
    }
}
pub fn lookup_item_type(code: &char) -> ItemType {
    match code {
        '0' => ItemType::TextFile,
        '1' => ItemType::GopherMap,
        '2' => ItemType::Nameserver,
        '3' => ItemType::Error,
        '4' => ItemType::MacBinary,
        '9' => ItemType::Binary,
        'i' => ItemType::Information,
        'I' => ItemType::Image,
        'h' => ItemType::HTML,
        _ => return ItemType::Unknown,
    }
}
