use std::io::BufRead;
use quick_xml::{events::{BytesStart, Event}, Reader};
use crate::utils::attributes_to_map;



#[derive(Debug)]
#[allow(dead_code)]
pub struct Metadata {
    pub section: String,
    pub items: Vec<MetadataItem>
}

impl Metadata {
    pub fn parse_one<B: BufRead>(
        e: &BytesStart,
        reader: &mut Reader<B>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let map = attributes_to_map(e)?;
        let section = map.get("section").cloned().ok_or("missing section")?;
        let items = MetadataItem::parse_many(reader, &mut buf)?;
        Ok(Metadata { section, items })
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct MetadataItem {
    pub name: String,
    pub text: String,
}

impl MetadataItem {
    pub fn parse_many<B: BufRead>(
        reader: &mut Reader<B>,
        buf: &mut Vec<u8>,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let mut fields: Vec<MetadataItem> = Vec::new();
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ref e) if e.name().as_ref() == b"item" => {
                    let map = attributes_to_map(e)?;
                    let mut text = String::new();
                    loop {
                        match reader.read_event_into(buf)? {
                            Event::Text(e) => {
                                text.push_str(&e.unescape()?.to_string());
                            }
                            Event::CData(e) => {
                                text.push_str(std::str::from_utf8(&e)?.trim());
                            }
                            Event::End(e) if e.name().as_ref() == b"item" => break,
                            Event::Eof => return Err("unexpected EOF inside <item>".into()),
                            _ => {}
                        }
                        buf.clear();
                    }
                    fields.push(MetadataItem {
                        name: map.get("name").cloned().ok_or("missing name")?,
                        text: text,
                    });
                }
                Event::End(ref e) if e.name().as_ref() == b"metadata" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(fields)
    }
}