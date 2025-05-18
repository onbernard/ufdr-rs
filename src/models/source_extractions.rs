use std::io::BufRead;

use quick_xml::{events::{Event}, Reader};
use crate::utils::attributes_to_map;



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
        let children = ExtractionInfo::parse_many(reader, &mut buf)?;
        Ok(SourceExtractions { infos: children })
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct ExtractionInfo {
    pub id: u32,
    pub name: String,
    pub is_custom_name: bool,
    pub dtype: String, // `type` is a Rust keyword
    pub device_name: String,
    pub full_name: String,
    pub index: u32,
    pub is_partial_data: bool,
}

impl ExtractionInfo {
    pub fn parse_many<B: BufRead>(
        reader: &mut Reader<B>,
        buf: &mut Vec<u8>,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let mut outp = Vec::new();
        loop {
            match reader.read_event_into(buf)? {
                Event::Empty(ref e) if e.name().as_ref() == b"extractionInfo" => {
                    let map = attributes_to_map(e)?;
                    outp.push(ExtractionInfo {
                        id: map.get("id").ok_or("missing id")?.parse()?,
                        name: map.get("name").cloned().ok_or("missing name")?,
                        is_custom_name: map
                            .get("isCustomName")
                            .map(|s| s.eq_ignore_ascii_case("true"))
                            .unwrap_or(false),
                        dtype: map.get("type").cloned().ok_or("missing type")?,
                        device_name: map.get("deviceName").cloned().ok_or("missing deviceName")?,
                        full_name: map.get("fullName").cloned().ok_or("missing fullName")?,
                        index: map.get("index").ok_or("missing index")?.parse()?,
                        is_partial_data: map
                            .get("IsPartialData")
                            .map(|s| s.eq_ignore_ascii_case("true"))
                            .unwrap_or(false),
                    });
                }
                Event::End(ref e) if e.name().as_ref() == b"sourceExtractions" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(outp)
    }
}