use std::{collections::HashMap, str};
use quick_xml::events::BytesStart;


pub fn attributes_to_map(
    e: &BytesStart,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();

    for attr in e.attributes() {
        let attr = attr?;
        let key = str::from_utf8(attr.key.as_ref())?.to_string();
        let val = attr.unescape_value()?.to_string();
        map.insert(key, val);
    }

    Ok(map)
}