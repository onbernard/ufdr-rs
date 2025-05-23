use std::io::BufRead;
use quick_xml::{events::{Event}, Reader};
use super::{ParseError, Timestamp};



#[derive(Debug, PartialEq)]
pub struct AccessInfo {
    pub timestamps: Vec<Timestamp>
}

impl AccessInfo {
    pub fn parse_one<B: BufRead>(
        reader: &mut Reader<B>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut timestamps = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"timestamp" => {
                    timestamps.push(Timestamp::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"accessInfo" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing accessInfo")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing accessInfo at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing accessInfo at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
            buf.clear();
        }
        Ok(AccessInfo { timestamps })
    }
}


#[cfg(test)]
mod test {
    use std::io::Cursor;
    use quick_xml::{events::Event, Reader};
    use crate::models::attributes_to_map;

    use super::*;

    fn test_access_info(xml_str: &str, expected: AccessInfo) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"accessInfo" => {
                    let uwu = AccessInfo::parse_one(&mut reader);
                    if let Ok(access_info) = uwu {
                        let known_keys: Vec<&str> = vec![
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown accessInfo attribute: {}", key);
                        }
                        assert_eq!(access_info, expected);
                        return Ok(());
                    } else {
                        return Err(format!("AccessInfo::parse_one error {:#?}", uwu));
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
    fn test_access_info_0() -> Result<(), String> {
        let xml_str = r#"
        <accessInfo>
            <timestamp name="CreationTime">2020-08-08T15:50:58.000+00:00</timestamp>
            <timestamp name="ModifyTime">2020-08-08T15:50:58.000+00:00</timestamp>
        </accessInfo>
        "#;
        test_access_info(xml_str, AccessInfo {
            timestamps: vec![
                Timestamp {
                    name: "CreationTime".to_string(),
                    text: "2020-08-08T15:50:58.000+00:00".to_string(),
                },
                Timestamp {
                    name: "ModifyTime".to_string(),
                    text: "2020-08-08T15:50:58.000+00:00".to_string(),
                }
            ]
        })
    }
}