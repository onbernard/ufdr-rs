use std::io::BufRead;
use quick_xml::{events::{Event}, Reader};
use crate::utils::attributes_to_map;



#[derive(Debug)]
#[allow(dead_code)]
pub struct CaseInformation {
    pub fields: Vec<CaseInformationField>
}

impl CaseInformation {
    pub fn parse_one<B: BufRead>(
        reader: &mut Reader<B>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let fields = CaseInformationField::parse_many(reader, &mut buf)?;
        Ok(CaseInformation { fields })
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct CaseInformationField {
    pub name: String,
    pub is_system: bool,
    pub is_required: bool,
    pub field_type: String,
    pub multiple_lines: bool,
    pub text: String,
}

impl CaseInformationField {
    pub fn parse_many<B: BufRead>(
        reader: &mut Reader<B>,
        buf: &mut Vec<u8>,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let mut fields = Vec::new();
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ref e) if e.name().as_ref() == b"field" => {
                    let map = attributes_to_map(e)?;
                    let mut text = String::new();
                    loop {
                        match reader.read_event_into(buf)? {
                            Event::Text(e) => {
                                text.push_str(&e.unescape()?.to_string());
                            }
                            Event::End(e) if e.name().as_ref() == b"field" => break,
                            Event::Eof => return Err("unexpected EOF inside <field>".into()),
                            _ => {}
                        }
                        buf.clear();
                    }
                    fields.push(CaseInformationField {
                        name: map.get("name").cloned().ok_or("missing name")?,
                        is_system: map
                            .get("isSystem")
                            .map(|s| s.eq_ignore_ascii_case("true"))
                            .unwrap_or(false),
                        is_required: map
                            .get("isRequired")
                            .map(|s| s.eq_ignore_ascii_case("true"))
                            .unwrap_or(false),
                        field_type: map.get("fieldType").cloned().ok_or("missing fieldType")?,
                        multiple_lines: map
                            .get("multipleLines")
                            .map(|s| s.eq_ignore_ascii_case("true"))
                            .unwrap_or(false),
                        text,
                    });
                }
                Event::End(ref e) if e.name().as_ref() == b"caseInformation" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(fields)
    }
}