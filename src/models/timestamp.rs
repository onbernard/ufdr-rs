use std::io::BufRead;
use quick_xml::{events::BytesStart, Reader};
use crate::utils::{attributes_to_map, read_text};



#[derive(Debug)]
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
    use super::*;

    fn test_timestamp(xml_str: &str, expected: Timestamp) -> Result<(), String> {
        todo!()
    }

    #[test]
    fn test_timestamp_0() -> Result<(), String> {
        let xml_str = r#"
        <timestamp name="CreationTime">2020-08-08T15:50:58.000+00:00</timestamp>
        "#;
        todo!()
    }

    #[test]
    fn test_timestamp_1() -> Result<(), String> {
        let xml_str = r#"
        <timestamp name="ModifyTime">2020-08-08T15:50:58.000+00:00</timestamp>
        "#;
        todo!()
    }

    #[test]
    fn test_timestamp_2() -> Result<(), String> {
        let xml_str = r#"
        <timestamp name="CreationTime">2020-08-08T15:50:58.000+00:00</timestamp>
        "#;
        todo!()
    }

    #[test]
    fn test_timestamp_3() -> Result<(), String> {
        let xml_str = r#"
        <timestamp name="ModifyTime">2020-08-08T15:50:58.000+00:00</timestamp>
        "#;
        todo!()
    }
}