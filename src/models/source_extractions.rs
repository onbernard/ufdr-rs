use std::{io::BufRead};

use quick_xml::{events::{BytesStart, Event}, Reader};
use crate::utils::attributes_to_map;



#[derive(Debug)]
struct ParseError {
    message: String,
}



#[derive(Debug)]
#[allow(dead_code)]
pub struct SourceExtractions {
    pub infos: Vec<ExtractionInfo>
}

impl SourceExtractions {
    pub fn parse_one<B: BufRead>(
        reader: &mut Reader<B>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut infos = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Empty(e) if e.name().as_ref() == b"extractionInfo" => {
                    infos.push(ExtractionInfo::parse_one(&e)?);
                }
                Event::End(e) if e.name().as_ref() == b"sourceExtractions" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing decodedData")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        panic!(
                            "unexpected text when parsing decodedData at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        )
                    }
                }
                unexpected => panic!(
                    "unexpected event when parsing decodedData at position {}: {:?}",
                    reader.buffer_position(),
                    unexpected
                )
            }
            buf.clear();
        }
        Ok(SourceExtractions { infos })
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct ExtractionInfo {
    pub id: u32,
    pub name: String,
    pub is_custom_name: String,
    pub dtype: String, // `type` is a Rust keyword
    pub device_name: String,
    pub full_name: String,
    pub index: u32,
    pub is_partial_data: String,
}

impl ExtractionInfo {
    pub fn parse_one(e: &BytesStart) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        Ok(ExtractionInfo {
            id: map.get("id").ok_or("missing id")?.parse()?,
            name: map.get("name").cloned().ok_or("missing name")?,
            is_custom_name: map.get("isCustomName").cloned().ok_or("missing isCustomName")?,
            dtype: map.get("type").cloned().ok_or("missing type")?,
            device_name: map.get("deviceName").cloned().ok_or("missing deviceName")?,
            full_name: map.get("fullName").cloned().ok_or("missing fullName")?,
            index: map.get("index").ok_or("missing index")?.parse()?,
            is_partial_data: map.get("IsPartialData").cloned().ok_or("missing IsPartialData")?,
        })
    }
}