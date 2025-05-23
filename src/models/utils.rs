use std::{collections::HashMap, io::BufRead, str};
use quick_xml::{events::{BytesStart, Event}, Reader};
use super::ParseError;


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


pub fn read_text<B: BufRead>(reader: &mut Reader<B>) -> Result<String, Box<dyn std::error::Error>> {
    let mut text = String::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Text(e) => {
                text.push_str(&e.unescape()?.to_string());
            }
            Event::CData(e) => {
                text.push_str(std::str::from_utf8(&e)?.trim());
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(Box::new(ParseError::new("unexpected EOF when parsing text")));
            },
            unexpected => {
                return Err(Box::new(ParseError::new(&format!(
                    "unexpected event when parsing text at position {}: {:?}",
                    reader.buffer_position(),
                    unexpected
                ))));
            }
        }
        buf.clear();
    }
    Ok(text)
}

