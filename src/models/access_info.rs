use std::io::BufRead;
use quick_xml::{events::{Event}, Reader};
use crate::models::Timestamp;



#[derive(Debug)]
pub struct AccessInfo {
    pub timestamps: Vec<Timestamp>
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