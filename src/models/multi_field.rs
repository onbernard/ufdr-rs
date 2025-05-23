use std::io::BufRead;

use quick_xml::{events::{BytesStart, Event}, Reader};

use crate::utils::{attributes_to_map, ParseError};

#[derive(Debug, PartialEq)]
pub struct MultiField {
    pub name: String,
    pub dtype: String,
}

impl MultiField {
    pub fn parse_one<B: BufRead>(e: &BytesStart, reader: &mut Reader<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Empty(e) if e.name().as_ref() == b"empty" => {}
                Event::End(e) if e.name().as_ref() == b"multiField" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing multiField")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing multiField at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing multiField at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
            buf.clear();
        }
        Ok(MultiField {
            name: map.get("name").cloned().ok_or("missing name")?,
            dtype: map.get("type").cloned().ok_or("missing type")?,
        })
    }

    pub fn parse_one_empty(e: &BytesStart) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        Ok(MultiField {
            name: map.get("name").cloned().ok_or("missing name")?,
            dtype: map.get("type").cloned().ok_or("missing type")?,
        })
    }
}


#[cfg(test)]
mod test {
    use std::{io::Cursor};

    use super::*;

    fn test_multi_field(xml_str: &str, expected: MultiField) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e))if e.name().as_ref() == b"multiField" => {
                    let uwu = MultiField::parse_one(&e, &mut reader);
                    if let Ok(uwuuwu) = uwu {
                        let known_keys: Vec<&str> = vec![
                            "name",
                            "type",
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown multiField attribute: {}", key);
                        }
                        assert_eq!(uwuuwu, expected);
                        return Ok(());
                    } else {
                        return Err(format!("MultiField::parse_one error {:#?}", uwu));
                    }
                },
                Ok(Event::Empty(e))  => {
                    let uwu = MultiField::parse_one_empty(&e);
                    if let Ok(uwuuwu) = uwu {
                        let known_keys: Vec<&str> = vec![
                            "name",
                            "type",
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown multiField attribute: {}", key);
                        }
                        assert_eq!(uwuuwu, expected);
                        return Ok(());
                    } else {
                        return Err(format!("MultiField::parse_one error {:#?}", uwu));
                    }
                }
                Ok(Event::Eof) => {
                    return Err("eof".to_string());
                }
                _ => (),
            }
            buf.clear();
        }
    }

    #[test]
    fn test_multi_field_0() -> Result<(), String> {
        let xml_str = r#"
        <multiField name="IPAddresses" type="String" />
        "#;
        test_multi_field(xml_str, MultiField {
            name: "IPAddresses".to_string(),
            dtype: "String".to_string()
        })
    }

    #[test]
    fn test_multi_field_1() -> Result<(), String> {
        let xml_str = r#"
        <multiField name="Notes" type="String">
            <empty />
        </multiField>
        "#;
        test_multi_field(xml_str, MultiField {
            name: "Notes".to_string(),
            dtype: "String".to_string()
        })
    }
}