use std::io::BufRead;
use quick_xml::{events::Event, Reader};
use super::ParseError;
use crate::models::Field;



#[derive(Debug, PartialEq)]
pub struct CaseInformation {
    pub fields: Vec<Field>
}

impl CaseInformation {
    pub fn parse_one<B: BufRead>(
        reader: &mut Reader<B>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut fields = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"field" => {
                    fields.push(Field::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"caseInformation" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing caseInformation")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing caseInformation at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing caseInformation at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
            buf.clear();
        }
        Ok(CaseInformation { fields })
    }
}


#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;


    #[test]
    fn test_case_information_0() -> Result<(), String> {
        let xml_str = r#"
        <caseInformation>
            <field name="Fall-Nummer" isSystem="True" isRequired="False" fieldType="CaseNumber" multipleLines="False">Case 001</field>
            <field name="Fallname" isSystem="True" isRequired="False" fieldType="CaseName" multipleLines="False">Super important case</field>
            <field name="Beweisnummer" isSystem="True" isRequired="False" fieldType="EvidenceNumber" multipleLines="False">001</field>
            <field name="Name d. Ermittlers" isSystem="False" isRequired="False" fieldType="None" multipleLines="False">Sherlock Holmes</field>
            <field name="Abteilung" isSystem="False" isRequired="False" fieldType="None" multipleLines="False">Department of Investigation</field>
            <field name="Ort" isSystem="False" isRequired="False" fieldType="None" multipleLines="False">Vienna</field>
            <field name="Beschuldigter" isSystem="False" isRequired="False" fieldType="None" multipleLines="False">Professor James Moriarty</field>
            <field name="PIN" isSystem="False" isRequired="False" fieldType="None" multipleLines="False">1337</field>
        </caseInformation>
        "#;
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"caseInformation" => {
                    let uwu = CaseInformation::parse_one(&mut reader);
                    if let Ok(case_information) = uwu {
                        assert_eq!(case_information, CaseInformation {
                            fields: vec![
                                Field {
                                    name: "Fall-Nummer".to_string(),
                                    is_system: Some("True".to_string()),
                                    is_required: Some("False".to_string()),
                                    field_type: Some("CaseNumber".to_string()),
                                    multiple_lines: Some("False".to_string()),
                                    dtype: None,
                                    text: "Case 001".to_string(),
                                    value: None,
                                },
                                Field {
                                    name: "Fallname".to_string(),
                                    is_system: Some("True".to_string()),
                                    is_required: Some("False".to_string()),
                                    field_type: Some("CaseName".to_string()),
                                    multiple_lines: Some("False".to_string()),
                                    dtype: None,
                                    text: "Super important case".to_string(),
                                    value: None,
                                },
                                Field {
                                    name: "Beweisnummer".to_string(),
                                    is_system: Some("True".to_string()),
                                    is_required: Some("False".to_string()),
                                    field_type: Some("EvidenceNumber".to_string()),
                                    multiple_lines: Some("False".to_string()),
                                    dtype: None,
                                    text: "001".to_string(),
                                    value: None,
                                },
                                Field {
                                    name: "Name d. Ermittlers".to_string(),
                                    is_system: Some("False".to_string()),
                                    is_required: Some("False".to_string()),
                                    field_type: Some("None".to_string()),
                                    multiple_lines: Some("False".to_string()),
                                    dtype: None,
                                    text: "Sherlock Holmes".to_string(),
                                    value: None,
                                },
                                Field {
                                    name: "Abteilung".to_string(),
                                    is_system: Some("False".to_string()),
                                    is_required: Some("False".to_string()),
                                    field_type: Some("None".to_string()),
                                    multiple_lines: Some("False".to_string()),
                                    dtype: None,
                                    text: "Department of Investigation".to_string(),
                                    value: None,
                                },
                                Field {
                                    name: "Ort".to_string(),
                                    is_system: Some("False".to_string()),
                                    is_required: Some("False".to_string()),
                                    field_type: Some("None".to_string()),
                                    multiple_lines: Some("False".to_string()),
                                    dtype: None,
                                    text: "Vienna".to_string(),
                                    value: None,
                                },
                                Field {
                                    name: "Beschuldigter".to_string(),
                                    is_system: Some("False".to_string()),
                                    is_required: Some("False".to_string()),
                                    field_type: Some("None".to_string()),
                                    multiple_lines: Some("False".to_string()),
                                    dtype: None,
                                    text: "Professor James Moriarty".to_string(),
                                    value: None,
                                },
                                Field {
                                    name: "PIN".to_string(),
                                    is_system: Some("False".to_string()),
                                    is_required: Some("False".to_string()),
                                    field_type: Some("None".to_string()),
                                    multiple_lines: Some("False".to_string()),
                                    dtype: None,
                                    text: "1337".to_string(),
                                    value: None,
                                }
                            ]
                        });
                        return Ok(());
                    } else {
                        return Err(format!("CaseInformation::parse_one error {:#?}", uwu));
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
}