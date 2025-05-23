use std::io::BufRead;
use quick_xml::{events::{BytesStart, Event}, Reader};
use crate::utils::{attributes_to_map, ParseError};
use super::{Field, ModelField, DataField, MultiField, MultiModelField};



#[derive(Debug, PartialEq)]
pub struct Model {
    pub dtype: String,
    pub id: String,
    pub deleted_state: String,
    pub decoding_confidence: String,
    pub is_related: String,
    pub extraction_id: u64,
    pub fields: Vec<Field>,
    pub multi_model_fields: Vec<MultiModelField>,
    pub model_fields: Vec<ModelField>,
    pub data_fields: Vec<DataField>,
    pub multi_fields: Vec<MultiField>,
}

impl Model {
    pub fn parse_one<B: BufRead>(e: &BytesStart, reader: &mut Reader<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        let mut buf = Vec::new();
        let mut fields = Vec::new();
        let mut multi_model_fields = Vec::new();
        let mut model_fields = Vec::new();
        let mut data_fields = Vec::new();
        let mut multi_fields = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"multiModelField" => {
                    multi_model_fields.push(MultiModelField::parse_one(&e, reader)?);
                }
                Event::Empty(e) if e.name().as_ref() == b"multiModelField" => {
                    multi_model_fields.push(MultiModelField::parse_one_empty(&e)?);
                }
                Event::Start(e) if e.name().as_ref() == b"multiField" => {
                    multi_fields.push(MultiField::parse_one(&e, reader)?);
                }
                Event::Empty(e) if e.name().as_ref() == b"multiField" => {
                    multi_fields.push(MultiField::parse_one_empty(&e)?);
                }
                Event::Start(e) if e.name().as_ref() == b"field" => {
                    fields.push(Field::parse_one(&e, reader)?);
                }
                Event::Start(e) if e.name().as_ref() == b"modelField" => {
                    model_fields.push(ModelField::parse_one(&e, reader)?);
                }
                Event::Start(e) if e.name().as_ref() == b"dataField" => {
                    data_fields.push(DataField::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"model" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing model")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing model at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing model at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
            buf.clear();
        }
        Ok(Model {
            dtype: map.get("type").cloned().ok_or("missing type")?,
            id: map.get("id").cloned().ok_or("missing id")?,
            deleted_state: map.get("deleted_state").cloned().ok_or("missing deleted_state")?,
            decoding_confidence: map.get("decoding_confidence").cloned().ok_or("missing decoding_confidence")?,
            is_related: map.get("isrelated").cloned().ok_or("missing isrelated")?,
            extraction_id: map.get("extractionId").cloned().ok_or("missing extractionId")?.parse()?,
            fields,
            multi_model_fields,
            model_fields,
            data_fields,
            multi_fields,
        })
    }
}


#[cfg(test)]
mod test {
    use std::{io::Cursor, vec};
    use crate::models::Value;
    use super::*;

    fn test_model(xml_str: &str, expected: Model) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"model" => {
                    let uwu = Model::parse_one(&e, &mut reader);
                    if let Ok(uwuuwu) = uwu {
                        let known_keys: Vec<&str> = vec![
                            "type",
                            "id",
                            "deleted_state",
                            "decoding_confidence",
                            "isrelated",
                            "extractionId",
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown model attribute: {}", key);
                        }
                        assert_eq!(uwuuwu, expected);
                        return Ok(());
                    } else {
                        return Err(format!("Model::parse_one error {:#?}", uwu));
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
    fn test_model_0() -> Result<(), String> {
        let xml_str = r#"
        <model type="InstantMessage" id="8204cd26-21cc-4510-9bc2-543253465415" deleted_state="Intact" decoding_confidence="High" isrelated="False" extractionId="1">
            <field name="UserMapping" type="Boolean">
                <value type="Boolean"><![CDATA[False]]></value>
            </field>
            <modelField name="From" type="Party">
            </modelField>
            <multiModelField name="To" type="Party" />
        </model>
        "#;
        test_model(xml_str, Model {
            dtype: "InstantMessage".to_string(),
            id: "8204cd26-21cc-4510-9bc2-543253465415".to_string(),
            deleted_state: "Intact".to_string(),
            decoding_confidence: "High".to_string(),
            is_related: "False".to_string(),
            extraction_id: 1,
            multi_fields: vec![],
            data_fields: vec![],
            model_fields: vec![ModelField {
                name: "From".to_string(),
                dtype: "Party".to_string(),
                models: vec![],
            }],
            fields: vec![
                Field {
                    name: "UserMapping".to_string(),
                    dtype: Some("Boolean".to_string()),
                    is_system: None,
                    is_required: None,
                    text: "".to_string(),
                    multiple_lines: None,
                    value: Some(Value {
                        dtype: "Boolean".to_string(),
                        text: "False".to_string(),
                    }),
                    field_type: None,
                }
            ],
            multi_model_fields: vec![
                MultiModelField {
                    name: "To".to_string(),
                    dtype: "Party".to_string(),
                    models: vec![],
                }
            ],
        })
    }
}