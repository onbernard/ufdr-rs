use std::{io::BufRead};

use quick_xml::{events::{BytesStart, Event}, Reader};
use crate::utils::{attributes_to_map, ParseError};



#[derive(Debug, PartialEq)]
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
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing sourceExtractions")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing sourceExtractions at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing sourceExtractions at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
            buf.clear();
        }
        Ok(SourceExtractions { infos })
    }
}


#[derive(Debug, PartialEq)]
pub struct ExtractionInfo {
    pub id: u32,
    pub name: String,
    pub is_custom_name: String,
    pub dtype: String,
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


#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_extraction_info() -> Result<(), String> {
        let xml_str = r#"
            <extractionInfo id="0" name="Logical" isCustomName="False" type="Logical" deviceName="Report" fullName="Cellebrite UFED Reports" index="0" IsPartialData="False" />
        "#;
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(e)) if e.name().as_ref() == b"extractionInfo" => {
                    if let Ok(extraction_info) = ExtractionInfo::parse_one(&e) {
                        assert_eq!(extraction_info, ExtractionInfo {
                            id: 0,
                            name: "Logical".to_string(),
                            is_custom_name: "False".to_string(),
                            dtype: "Logical".to_string(),
                            device_name: "Report".to_string(),
                            full_name: "Cellebrite UFED Reports".to_string(),
                            index: 0,
                            is_partial_data: "False".to_string(),
                        });
                        return Ok(());
                    } else {
                        return Err("ExtractionInfo::parse_one error".to_string());
                    }
                },
                Ok(Event::Eof) => {
                    return Err("eof".to_string());
                }
                _ => (),
            }
            buf.clear();
        }
    }

    #[test]
    fn test_source_extractions() -> Result<(), String> {
        let xml_str = r#"
        <sourceExtractions>
            <extractionInfo id="0" name="Logical" isCustomName="False" type="Logical" deviceName="Report" fullName="Cellebrite UFED Reports" index="0" IsPartialData="False" />
        </sourceExtractions>
        "#;
                let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"sourceExtractions" => {
                    if let Ok(source_extraction) = SourceExtractions::parse_one(&mut reader) {
                        assert_eq!(source_extraction, SourceExtractions {
                            infos: vec![ExtractionInfo {
                                id: 0,
                                name: "Logical".to_string(),
                                is_custom_name: "False".to_string(),
                                dtype: "Logical".to_string(),
                                device_name: "Report".to_string(),
                                full_name: "Cellebrite UFED Reports".to_string(),
                                index: 0,
                                is_partial_data: "False".to_string(),
                            }]
                        });
                        return Ok(());
                    } else {
                        return Err("sourceExtractions::parse_one error".to_string());
                    }
                },
                Ok(Event::Eof) => {
                    return Err("eof".to_string());
                }
                _ => (),
            }
            buf.clear();
        }
    }
}