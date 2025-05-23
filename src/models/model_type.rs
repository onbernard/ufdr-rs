use std::{io::BufRead};
use quick_xml::{events::{BytesStart, Event}, Reader};
use crate::utils::{attributes_to_map, ParseError};
use super::Model;



#[derive(Debug, PartialEq)]
pub struct ModelType {
    pub dtype: String,
    pub models: Vec<Model>,
}

impl ModelType {
    pub fn parse_one<B: BufRead>(
        e: &BytesStart,
        reader: &mut Reader<B>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut models = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"model" => {
                    models.push(Model::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"modelType" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing modelType")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing modelType at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing modelType at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
        }
        let map = attributes_to_map(e)?;
        Ok(ModelType {
            dtype: map.get("type").cloned().ok_or("missing type")?,
            models,
        })
    }
}


#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::models::{Field, Value};

    use super::*;

    fn test_model_type(xml_str: &str, expected: ModelType) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"modelType" => {
                    let uwu = ModelType::parse_one(&e, &mut reader);
                    if let Ok(uwuuwu) = uwu {
                        let known_keys: Vec<&str> = vec![
                            "type",
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown modelType attribute: {}", key);
                        }
                        assert_eq!(uwuuwu, expected);
                        return Ok(());
                    } else {
                        return Err(format!("ModelType::parse_one error {:#?}", uwu));
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
    fn test_model_type_0() -> Result<(), String> {
        let xml_str = r#"
        <modelType type="Chat">
            <model type="Chat" id="9423c49d-2696-4534-a54f-b5eec3e92e77" deleted_state="Intact" decoding_confidence="High" isrelated="False" extractionId="1">
                <field name="UserMapping" type="Boolean">
                    <value type="Boolean"><![CDATA[False]]></value>
                </field>
            </model>
        </modelType>
        "#;
        test_model_type(xml_str, ModelType {
            dtype: "Chat".to_string(),
            models: vec![Model {
                dtype: "Chat".to_string(),
                id: "9423c49d-2696-4534-a54f-b5eec3e92e77".to_string(),
                deleted_state: "Intact".to_string(),
                decoding_confidence: "High".to_string(),
                is_related: "False".to_string(),
                extraction_id: 1,
                multi_model_fields: vec![],
                model_fields: vec![],
                multi_fields: vec![],
                data_fields: vec![],
                fields: vec![Field {
                    name: "UserMapping".to_string(),
                    dtype: Some("Boolean".to_string()),
                    is_system: None,
                    is_required: None,
                    multiple_lines: None,
                    text: "".to_string(),
                    field_type: None,
                    value: Some(Value {
                        dtype: "Boolean".to_string(),
                        text: "False".to_string(),
                    })
                }]
            }]
        })
    }
}