use std::{io::BufRead};
use quick_xml::{events::{BytesStart, Event}, Reader};
use crate::utils::attributes_to_map;
use crate::models::Metadata;



#[derive(Debug)]
#[allow(dead_code)]
pub struct TaggedFiles {
    files: Vec<File>
}

impl TaggedFiles {
    pub fn parse_one<B: BufRead>(
        reader: &mut Reader<B>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut files: Vec<File> = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"file" => {
                    files.push(File::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"taggedFiles" => break,
                Event::Eof => break,
                _ => {}
            }
        }
        Ok(TaggedFiles { files: files })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
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
                Event::Eof => break,
                _ => {}
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


#[derive(Debug)]
#[allow(dead_code)]
pub struct AccessInfo {
    timestamps: Vec<Timestamp>
}

impl AccessInfo {
    pub fn parse_one<B: BufRead>(
        reader: &mut Reader<B>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut timestamps: Vec<Timestamp> = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"timestamp" => {
                    timestamps.push(Timestamp::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"accessInfo" => break,
                Event::Eof => break,
                _ => {}
            }
        }
        Ok(AccessInfo { timestamps })
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct Timestamp {
    name: String,
    text: String,
}

impl Timestamp {
    pub fn parse_one<B: BufRead>(e: &BytesStart, reader: &mut Reader<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        let mut text = String::new();
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Text(e) => {
                    text.push_str(&e.unescape()?.to_string());
                }
                Event::End(e) if e.name().as_ref() == b"timestamp" => break,
                Event::Eof => return Err("unexpected EOF inside <timestamp>".into()),
                _ => {}
            }
            buf.clear();
        }
        Ok(Timestamp {
            name: map.get("name").cloned().ok_or("missing name")?,
            text,
        })
    }
}