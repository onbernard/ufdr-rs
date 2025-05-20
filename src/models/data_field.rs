use std::io::BufRead;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::models::Source;
use crate::utils::attributes_to_map;



#[derive(Debug, PartialEq)]
pub struct DataField {
    pub name: String,
    pub dtype: String,
    pub sources: Vec<Source>
}

impl DataField {
    pub fn parse_one<B: BufRead>(e: &BytesStart, reader: &mut Reader<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        let mut buf = Vec::new();
        let mut sources = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Empty(e) if e.name().as_ref() == b"source" => {
                    sources.push(Source::parse_one(&e)?);
                }
                Event::End(e) if e.name().as_ref() == b"dataField" => break,
                Event::Eof => panic!("unexpected eof when parsing dataField"),
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        panic!(
                            "unexpected text when parsing dataField at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        )
                    }
                }
                unexpected => panic!(
                    "unexpected event when parsing dataField at position {}: {:?}",
                    reader.buffer_position(),
                    unexpected
                )
            }
            buf.clear();
        }
        Ok(DataField {
            name: map.get("name").cloned().ok_or("missing name")?,
            dtype: map.get("type").cloned().ok_or("missing type")?,
            sources,
        })
    }
}
