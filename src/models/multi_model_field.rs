use std::io::BufRead;
use quick_xml::{events::{BytesStart, Event}, Reader};
use crate::utils::{attributes_to_map, ParseError};
use super::Model;



#[derive(Debug, PartialEq)]
pub struct MultiModelField {
    pub name: String,
    pub dtype: String,
    pub models: Vec<Model>
}

impl MultiModelField {
    pub fn parse_one<B: BufRead>(e: &BytesStart, reader: &mut Reader<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        let mut models = Vec::new();
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"model" => {
                    models.push(Model::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"multiModelField" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing multiModelField")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing multiModelField at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing multiModelField at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
            buf.clear();
        }
        
        Ok(MultiModelField {
            name: map.get("name").cloned().ok_or("missing name")?,
            dtype: map.get("type").cloned().ok_or("missing type")?,
            models,
        })
    }

    pub fn parse_one_empty(e: &BytesStart) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        Ok(Self {
            name: map.get("name").cloned().ok_or("missing name")?,
            dtype: map.get("type").cloned().ok_or("missing type")?,
            models: vec![],
        })
    }
}