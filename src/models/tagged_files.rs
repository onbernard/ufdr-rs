use std::io::BufRead;
use quick_xml::{events::{BytesStart, Event}, Reader};
use crate::utils::attributes_to_map;



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
    fs: String,
    fsid: String,
    path: String,
    size: u64,
    id: String,
    extraction_id: u64,
    deleted: String,
    embedded: String,
    is_related: String,
    access_info: Option<AccessInfo>,
}

impl File {
    pub fn parse_one<B: BufRead>(
        e: &BytesStart,
        reader: &mut Reader<B>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut access_info: Option<AccessInfo> = None;
        loop {
            match reader.read_event_into(&mut buf)? {

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
            access_info: access_info,
        })
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct AccessInfo {
    timestamps: Vec<Timestamp>
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Timestamp {
    name: String,
    text: String,
}