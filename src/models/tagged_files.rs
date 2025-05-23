use std::{io::BufRead};
use quick_xml::{Reader, events::Event};
use super::{File, ParseError};



#[derive(Debug, PartialEq)]
pub struct TaggedFiles {
    pub files: Vec<File>
}

impl TaggedFiles {
    pub fn parse_one<B: BufRead>(
        reader: &mut Reader<B>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut files = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"file" => {
                    files.push(File::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"taggedFiles" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing taggedFiles")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing taggedFiles at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing taggedFiles at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
            buf.clear();
        }
        Ok(TaggedFiles { files })
    }
}


#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::models::{attributes_to_map, AccessInfo, Item, Metadata, Timestamp};

    use super::*;

    fn test_tagged_files(xml_str: &str, expected: TaggedFiles) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf  = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"taggedFiles" => {
                    let uwu = TaggedFiles::parse_one(&mut reader);
                    if let Ok(tagged_files) = uwu {
                        let known_keys: Vec<&str> = vec![
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown taggedFiles attribute: {}", key);
                        }
                        assert_eq!(tagged_files, expected);
                        return Ok(());
                    } else {
                        return Err(format!("TaggedFiles::parse_one error {:#?}", uwu));
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
    fn test_tagged_files_0() -> Result<(), String> {
        let xml_str = r#"
        <taggedFiles>
            <file fs="iPhone of Prof Moriarty" fsid="01234567-f1ea-421a-9ea3-0123456789ab" path="/another/path/on/the/mobile/phone/4454825783_dbcb233af5_b.jpg" size="215418" id="b231534c-b43b-477d-863e-412342341" extractionId="1" deleted="Intact" embedded="false" isrelated="False">
                <accessInfo>
                    <timestamp name="CreationTime">2020-08-08T15:50:58.000+00:00</timestamp>
                    <timestamp name="ModifyTime">2020-08-08T15:50:58.000+00:00</timestamp>
                </accessInfo>
                <metadata section="File">
                    <item name="Local Path"><![CDATA[files\Image\4454825783_dbcb233af5_b.jpg]]></item>
                </metadata>
            </file>
        </taggedFiles>
        "#;
        test_tagged_files(xml_str, TaggedFiles {
            files: vec![File {
                fs: "iPhone of Prof Moriarty".to_string(),
                fsid: "01234567-f1ea-421a-9ea3-0123456789ab".to_string(),
                path: "/another/path/on/the/mobile/phone/4454825783_dbcb233af5_b.jpg".to_string(),
                size: 215418,
                id: "b231534c-b43b-477d-863e-412342341".to_string(),
                extraction_id: 1,
                deleted: "Intact".to_string(),
                embedded: "false".to_string(),
                is_related: "False".to_string(),
                access_info: Some(AccessInfo { timestamps: vec![
                    Timestamp {
                        name: "CreationTime".to_string(),
                        text: "2020-08-08T15:50:58.000+00:00".to_string(),
                    },
                    Timestamp {
                        name: "ModifyTime".to_string(),
                        text: "2020-08-08T15:50:58.000+00:00".to_string(),
                    }
                ] }),
                metadata: vec![
                    Metadata {
                        section: "File".to_string(),
                        items: vec![Item {
                            name: "Local Path".to_string(),
                            group: None,
                            id: None,
                            source_extraction: None,
                            text: r"files\Image\4454825783_dbcb233af5_b.jpg".to_string()
                        }]
                    }
                ],
            }]
        })
    }
}