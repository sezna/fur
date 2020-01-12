use super::{GopherParseError, GopherResponse, ItemType};
use std::collections::HashMap;

pub struct GopherMap {
    menu_items: Vec<MenuItem>,
    pub options: HashMap<usize, MenuItem>,
}
impl GopherMap {
    pub fn new() -> GopherMap {
        GopherMap {
            menu_items: Vec::new(),
            options: HashMap::new(),
        }
    }
    /// Given an option selection, return the corresponding menu item.
    pub fn select_option(&self, selection: usize) -> &MenuItem {
        self.options.get(&selection).expect("Failed to select option")
    }
}

impl GopherResponse for GopherMap {
    const ITEM_TYPE: char = '1';
    fn from_str(input: &str) -> Result<Self, GopherParseError> {
        let mut to_return = GopherMap::new();
        let mut counter = 0usize;
        for line in input.lines() {
            let mut entries = line
                .split('\t')
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            let mut item_iter = entries[0].chars();
            if entries[0].chars().collect::<Vec<char>>().len() == 0 {
                println!("null entry {}", input);
            }
            let first_letter = item_iter.next().expect("Entry has zero letters");
            let item_type = lookup_item_type(&first_letter);
            if item_type == ItemType::Unknown {
                to_return.menu_items.push(MenuItem {
                    item_type,
                    display_string: input.to_string(),
                    selector: String::new(),
                    hostname: String::new(),
                    port: 0usize,
                });
            } else {
                entries[0] = item_iter.collect::<String>();
                let item = MenuItem {
                    item_type,
                    display_string: entries[0].clone(),
                    selector: entries[1].clone(),
                    hostname: entries[2].clone(),
                    port: str::parse::<usize>(entries[3].as_str()).expect("failed to parse port"),
                };
                match item.item_type {
                    ItemType::Information => (),
                    _ => {
                        to_return.options.insert(counter, item.clone());
                        counter += 1;
                    }
                };
                to_return.menu_items.push(item);
            }
        }
        Ok(to_return)
    }
    fn render(&self) -> String {
        let mut output_strings: Vec<String> = Vec::new();
        let mut counter = 0;
        for item in self.menu_items.iter() {
            match item.item_type {
                ItemType::Error => output_strings.push(format!(
                    "{}: {}",
                    item.item_type.to_string(),
                    item.display_string
                )),
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

fn lookup_item_type(code: &char) -> ItemType {
    match code {
        '0' => ItemType::TextFile,
        '1' => ItemType::GopherMap,
        '2' => ItemType::Nameserver,
        '3' => ItemType::Error,
        '4' => ItemType::MacBinary,
        '9' => ItemType::Binary,
        'i' => ItemType::Information,
        'I' => ItemType::Image,
        _ => return ItemType::Unknown,
    }
}

#[derive(Clone)]
pub struct MenuItem {
    pub item_type: ItemType,
    pub display_string: String,
    pub selector: String,
    pub hostname: String,
    pub port: usize,
}
