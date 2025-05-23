use std::io::BufRead;
use quick_xml::{Reader, events::{BytesStart, Event}};
use super::{attributes_to_map, ParseError, Source};



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
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing dataField")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing dataField at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing dataField at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
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


#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    fn test_data_field(xml_str: &str, expected: DataField) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"dataField" => {
                    let uwu = DataField::parse_one(&e, &mut reader);
                    if let Ok(df) = uwu {
                        let known_keys: Vec<&str> = vec![
                            "name",
                            "type",
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown dataField attribute: {}", key);
                        }
                        assert_eq!(df, expected);
                        return Ok(());
                    } else {
                        return Err(format!("DataField::parse_one error {:#?}", uwu));
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
    fn test_data_field_0() -> Result<(), String> {
        let xml_str = r#"
        <dataField name="Data" type="MemoryRange">
            <source length="11159817" />
        </dataField>
        "#;
        test_data_field(xml_str, DataField {
            name: "Data".to_string(),
            dtype: "MemoryRange".to_string(),
            sources: vec![Source { length: 11159817 }],
        })
    }
}