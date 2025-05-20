use std::io::BufRead;
use quick_xml::{events::{BytesStart, Event}, Reader};
use crate::utils::{attributes_to_map, ParseError};



#[derive(Debug, PartialEq)]
pub struct Images {
    pub images: Vec<Image>
}

impl Images {
    pub fn parse_one<B: BufRead>(
        reader: &mut Reader<B>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut images = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Empty(e) if e.name().as_ref() == b"image" => {
                    images.push(Image::parse_one(&e)?);
                }
                Event::End(e) if e.name().as_ref() == b"images" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing images")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing images at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing images at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
            buf.clear();
        }
        Ok(Images { images })
    }
}


#[derive(Debug, PartialEq)]
pub struct Image {
    pub key: String,
    pub path: String,
    pub size: u64,
    pub dtype: String,
    pub verify: String,
    pub extraction_id: u64,
}

impl Image {
    pub fn parse_one(e: &BytesStart) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        Ok(Image {
            key: map.get("key").ok_or("missing key")?.parse()?,
            path: map.get("path").ok_or("missing path")?.parse()?,
            size: map.get("size").ok_or("missing size")?.parse()?,
            dtype: map.get("type").ok_or("missing type")?.parse()?,
            verify: map.get("verify").ok_or("missing verify")?.parse()?,
            extraction_id: map.get("extractionId").ok_or("missing extractionId")?.parse()?,
        })
    }
}


#[cfg(test)]
mod test {
    use std::io::Cursor;
    use quick_xml::{events::Event, Reader};
    use super::*;

    fn test_images(xml_str: &str, expected: Images) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"images" => {
                    let uwu = Images::parse_one(&mut reader);
                    if let Ok(images) = uwu {
                        let known_keys: Vec<&str> = vec![
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown images attribute: {}", key);
                        }
                        assert_eq!(images, expected);
                        return Ok(());
                    } else {
                        return Err(format!("Images::parse_one error {:#?}", uwu));
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
    fn test_images_0() -> Result<(), String> {
        let xml_str = r#"
        <images>
            <image key="FileDump" path="iPhoneBackup.tar" size="12345678" type="File" verify="NoSourceValues" extractionId="2" />
        </images>
        "#;
        test_images(xml_str, Images {
            images: vec![Image {
            key: "FileDump".to_string(),
            path: "iPhoneBackup.tar".to_string(),
            size: 12345678,
            dtype: "File".to_string(),
            verify: "NoSourceValues".to_string(),
            extraction_id: 2,
        }]
        })
    }

    fn test_image(xml_str: &str, expected: Image) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(e)) if e.name().as_ref() == b"image" => {
                    let uwu = Image::parse_one(&e);
                    if let Ok(image) = uwu {
                        let known_keys = vec![
                            "key",
                            "path",
                            "size",
                            "type",
                            "verify",
                            "extractionId",
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown image attribute: {}", key);
                        }
                        assert_eq!(image, expected);
                        return Ok(());
                    } else {
                        return Err(format!("Image::parse_one error {:#?}", uwu));
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
    fn test_image_0() -> Result<(), String> {
        let xml_str = r#"
        <image key="FileDump" path="iPhoneBackup.tar" size="12345678" type="File" verify="NoSourceValues" extractionId="2" />
        "#;
        test_image(xml_str, Image {
            key: "FileDump".to_string(),
            path: "iPhoneBackup.tar".to_string(),
            size: 12345678,
            dtype: "File".to_string(),
            verify: "NoSourceValues".to_string(),
            extraction_id: 2,
        })
    }
}