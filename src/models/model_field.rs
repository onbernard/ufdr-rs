use std::io::BufRead;
use quick_xml::{events::{BytesStart, Event}, Reader};
use crate::utils::{attributes_to_map, ParseError};
use crate::models::{Model};



#[derive(Debug, PartialEq)]
pub struct ModelField {
    pub name: String,
    pub dtype: String,
    pub models: Vec<Model>,
}

impl ModelField {
    pub fn parse_one<B: BufRead>(e: &BytesStart, reader: &mut Reader<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        let mut buf = Vec::new();
        let mut models = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"model" => {
                    models.push(Model::parse_one(&e, reader)?);
                }
                Event::Empty(e) if e.name().as_ref() == b"empty" => {}
                Event::End(e) if e.name().as_ref() == b"modelField" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing modelField")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing modelField at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing modelField at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
            buf.clear();
        }
        Ok(ModelField {
            name: map.get("name").cloned().ok_or("missing name")?,
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

    fn test_model_field(xml_str: &str, expected: ModelField) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"modelField" => {
                    let uwu = ModelField::parse_one(&e, &mut reader);
                    if let Ok(uwuuwu) = uwu {
                        let known_keys: Vec<&str> = vec![
                            "name",
                            "type",
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown modelField attribute: {}", key);
                        }
                        assert_eq!(uwuuwu, expected);
                        return Ok(());
                    } else {
                        return Err(format!("ModelField::parse_one error {:#?}", uwu));
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
    fn test_model_field_0() -> Result<(), String> {
        let xml_str = r#"
        <modelField name="From" type="Party">
            <model type="Party" id="2affbd71-3369-4375-8da7-67674563456" deleted_state="Unknown" decoding_confidence="High" isrelated="False" extractionId="1">
                <field name="UserMapping" type="Boolean">
                    <value type="Boolean"><![CDATA[False]]></value>
                </field>
            </model>
        </modelField>
        "#;
        test_model_field(xml_str, ModelField {
            name: "From".to_string(),
            dtype: "Party".to_string(),
            models: vec![
                Model {
                    dtype: "Party".to_string(),
                    id: "2affbd71-3369-4375-8da7-67674563456".to_string(),
                    deleted_state: "Unknown".to_string(),
                    decoding_confidence: "High".to_string(),
                    is_related: "False".to_string(),
                    extraction_id: 1,
                    fields: vec![Field {
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
                    }],
                    model_fields: vec![],
                    multi_model_fields: vec![],
                    multi_fields: vec![],
                    data_fields: vec![],
                }
            ]
        })
    }
}