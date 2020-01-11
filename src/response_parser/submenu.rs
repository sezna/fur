use super::{GopherParseError, GopherResponse, ItemType};

pub struct Submenu {
    pub menu_items: Vec<MenuItem>,
}
impl Submenu {
    pub fn new() -> Submenu {
        Submenu {
            menu_items: Vec::new(),
        }
    }
    pub fn peruse(&self) -> String {
        let mut output_strings: Vec<String> = Vec::new();
        let mut counter = 0;
        for item in self.menu_items.iter() {
            match item.item_type {
                ItemType::Information => output_strings.push(format!(
                    "     {}: {}",
                    item.item_type.to_string(),
                    item.display_string
                )),
                _ => {
                    output_strings.push(format!(
                        "({:02}) {}: {}",
                        counter,
                        item.item_type.to_string(),
                        item.display_string
                    ));
                    counter += 1;
                }
            }
        }
        format!(
            "===========================================================\n{}",
            output_strings.join("\n")
        )
    }
}

impl GopherResponse for Submenu {
    const ItemType: char = '1';
    fn from_str(input: &str) -> Result<Self, GopherParseError> {
        let mut to_return = Submenu::new();
        for line in input.lines() {
            let mut entries = line
                .split('\t')
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            let mut item_iter = entries[0].chars();
            let first_letter = item_iter.next().unwrap();
            let item_type = lookup_item_type(&first_letter).expect("unimpl'd item type");
            entries[0] = item_iter.collect::<String>();
            to_return.menu_items.push(MenuItem {
                item_type,
                display_string: entries[0].clone(),
                selector: entries[1].clone(),
                hostname: entries[2].clone(),
                port: str::parse::<usize>(entries[3].as_str()).expect("failed to parse port"),
            });
        }
        Ok(to_return)
    }
}

fn lookup_item_type(code: &char) -> Option<ItemType> {
    Some(match code {
        '0' => ItemType::TextFile,
        '1' => ItemType::Submenu,
        '2' => ItemType::Nameserver,
        'i' => ItemType::Information,
        _ => return None,
    })
}

pub struct MenuItem {
    item_type: ItemType,
    display_string: String,
    selector: String,
    hostname: String,
    port: usize,
}
