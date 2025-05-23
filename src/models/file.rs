use std::io::BufRead;
use quick_xml::{events::{BytesStart, Event}, Reader};
use super::{attributes_to_map, AccessInfo, Metadata, ParseError};


#[derive(Debug, PartialEq)]
pub struct File {
    pub fs: String,
    pub fsid: String,
    pub path: String,
    pub size: u64,
    pub id: String,
    pub extraction_id: u64,
    pub deleted: String,
    pub embedded: String,
    pub is_related: String,
    pub access_info: Option<AccessInfo>,
    pub metadata: Vec<Metadata>
}

impl File {
    pub fn parse_one<B: BufRead>(
        e: &BytesStart,
        reader: &mut Reader<B>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut access_info: Option<AccessInfo> = None;
        let mut metadata: Vec<Metadata> = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"accessInfo" => {
                    access_info = Some(AccessInfo::parse_one(reader)?);
                }
                Event::Start(e) if e.name().as_ref() == b"metadata" => {
                    metadata.push(Metadata::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"file" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing file")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing file at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing file at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
        }
        let map = attributes_to_map(e)?;
        Ok(File {
            fs: map.get("fs").cloned().ok_or("missing fs")?,
            fsid: map.get("fsid").cloned().ok_or("missing fsid")?,
            path: map.get("path").cloned().ok_or("missing path")?,
            size: map.get("size").cloned().ok_or("missing size")?.parse()?,
            id: map.get("id").cloned().ok_or("missing id")?,
            extraction_id: map.get("extractionId").cloned().ok_or("missing extractionId")?.parse()?,
            deleted: map.get("deleted").cloned().ok_or("missing deleted")?,
            embedded: map.get("embedded").cloned().ok_or("missing embedded")?,
            is_related: map.get("isrelated").cloned().ok_or("missing isrelated")?,
            access_info,
            metadata
        })
    }
}


#[cfg(test)]
mod test {
    use std::io::Cursor;
    use crate::models::{Item, Timestamp};
    use super::*;

    fn test_file(xml_str: &str, expected: File) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf  = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"file" => {
                    let uwu = File::parse_one(&e, &mut reader);
                    if let Ok(file) = uwu {
                        let known_keys: Vec<&str> = vec![
                            "fs",
                            "fsid",
                            "path",
                            "size",
                            "id",
                            "extractionId",
                            "deleted",
                            "embedded",
                            "isrelated",
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown file attribute: {}", key);
                        }
                        assert_eq!(file, expected);
                        return Ok(());
                    } else {
                        return Err(format!("File::parse_one error {:#?}", uwu));
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
    fn test_file_0() -> Result<(), String> {
        let xml_str = r#"
        <file fs="iPhone of Prof Moriarty" fsid="01234567-f1ea-421a-9ea3-0123456789ab" path="/another/path/on/the/mobile/phone/4454825783_dbcb233af5_b.jpg" size="215418" id="b231534c-b43b-477d-863e-412342341" extractionId="1" deleted="Intact" embedded="false" isrelated="False">
            <accessInfo>
                <timestamp name="CreationTime">2020-08-08T15:50:58.000+00:00</timestamp>
                <timestamp name="ModifyTime">2020-08-08T15:50:58.000+00:00</timestamp>
            </accessInfo>
            <metadata section="File">
                <item name="Local Path"><![CDATA[files\Image\4454825783_dbcb233af5_b.jpg]]></item>
            </metadata>
        </file>
        "#;
        test_file(xml_str, File {
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
        })
    }
}