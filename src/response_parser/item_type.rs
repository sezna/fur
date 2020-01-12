#[derive(Clone)]
pub enum ItemType {
    TextFile,
    GopherMap,
    Nameserver,
    Binary,
    Error,
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
            ItemType::Error => " ERR",
        }
        .to_string()
    }
}
