use quick_xml::events::BytesStart;
use crate::utils::attributes_to_map;



#[derive(Debug, PartialEq)]
pub struct Source {
    pub length: u64,
}

impl Source {
    pub fn parse_one(e: &BytesStart) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        Ok(Source { length: map.get("length").cloned().ok_or("missing length")?.parse()? })
    }
}