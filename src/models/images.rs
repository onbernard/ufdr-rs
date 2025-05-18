use std::io::BufRead;
use quick_xml::{events::{Event}, Reader};
use crate::utils::attributes_to_map;



#[derive(Debug)]
#[allow(dead_code)]
pub struct Images {
    pub images: Vec<Image>
}

impl Images {
    pub fn parse_one<B: BufRead>(
        reader: &mut Reader<B>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let images = Image::parse_many(reader, &mut buf)?;
        Ok(Images { images })
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct Image {
    key: String,
    path: String,
    size: u64,
    dtype: String,
    verify: String,
    extraction_id: u64,
}

impl Image {
    pub fn parse_many<B: BufRead>(
        reader: &mut Reader<B>,
        buf: &mut Vec<u8>,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let mut outp: Vec<Image> = Vec::new();
        loop {
            match reader.read_event_into(buf)? {
                Event::Empty(ref e) if e.name().as_ref() == b"image" => {
                    let map = attributes_to_map(e)?;
                    outp.push(Image {
                        key: map.get("key").ok_or("missing key")?.parse()?,
                        path: map.get("path").ok_or("missing path")?.parse()?,
                        size: map.get("size").ok_or("missing size")?.parse()?,
                        dtype: map.get("type").ok_or("missing type")?.parse()?,
                        verify: map.get("verify").ok_or("missing verify")?.parse()?,
                        extraction_id: map.get("extractionId").ok_or("missing extractionId")?.parse()?,
                    });
                }
                Event::End(ref e) if e.name().as_ref() == b"images" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(outp)
    }
}