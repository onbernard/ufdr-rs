use quick_xml::events::BytesStart;
use super::attributes_to_map;



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


#[cfg(test)]
mod test {
    use std::io::Cursor;
    use quick_xml::{events::Event, Reader};
    use super::*;

    fn test_source(xml_str: &str, expected: Source) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(e)) if e.name().as_ref() == b"source" => {
                    let uwu = Source::parse_one(&e);
                    if let Ok(source) = uwu {
                        let known_keys = vec![
                            "length",
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown source attribute: {}", key);
                        }
                        assert_eq!(source, expected);
                        return Ok(());
                    } else {
                        return Err(format!("Source::parse_one error {:#?}", uwu));
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
    fn test_source_0() -> Result<(), String> {
        let xml_str = r#"
        <source length="11159817" />
        "#;
        test_source(xml_str, Source { length: 11159817 })
    }
}