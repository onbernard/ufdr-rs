use std::io::BufRead;
use quick_xml::{events::BytesStart, Reader};
use super::{attributes_to_map, read_text};



#[derive(Debug, PartialEq)]
pub struct Timestamp {
    pub name: String,
    pub text: String,
}

impl Timestamp {
    pub fn parse_one<B: BufRead>(e: &BytesStart, reader: &mut Reader<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        let text = read_text(reader)?;
        Ok(Timestamp {
            name: map.get("name").cloned().ok_or("missing name")?,
            text,
        })
    }
}


#[cfg(test)]
mod test {
    use std::io::Cursor;
    use quick_xml::{events::Event, Reader};
    use super::*;

    fn test_timestamp(xml_str: &str, expected: Timestamp) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"timestamp" => {
                    let uwu = Timestamp::parse_one(&e, &mut reader);
                    if let Ok(timestamp) = uwu {
                        let known_keys = vec![
                            "name",
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown timestamp attribute: {}", key);
                        }
                        assert_eq!(timestamp, expected);
                        return Ok(());
                    } else {
                        return Err(format!("Timestamp::parse_one error {:#?}", uwu));
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
    fn test_timestamp_0() -> Result<(), String> {
        let xml_str = r#"
        <timestamp name="CreationTime">2020-08-08T15:50:58.000+00:00</timestamp>
        "#;
        test_timestamp(xml_str, Timestamp {
            name: "CreationTime".to_string(),
            text: "2020-08-08T15:50:58.000+00:00".to_string(),
        })
    }

    #[test]
    fn test_timestamp_1() -> Result<(), String> {
        let xml_str = r#"
        <timestamp name="ModifyTime">2020-08-08T15:50:58.000+00:00</timestamp>
        "#;
        test_timestamp(xml_str, Timestamp {
            name: "ModifyTime".to_string(),
            text: "2020-08-08T15:50:58.000+00:00".to_string(),
        })
    }
}