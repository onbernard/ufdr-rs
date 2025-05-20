use core::panic;
use std::{io::BufRead};
use pyo3::pyclass;
use quick_xml::{events::{BytesStart, Event}, Reader};
use crate::utils::attributes_to_map;
use crate::models::{DataField, Field};


#[derive(Debug, PartialEq)]
#[pyclass]
pub struct DecodedData {
    pub model_types: Vec<ModelType>
}

impl DecodedData {
    pub fn parse_one<B: BufRead>(
        reader: &mut Reader<B>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut model_types = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"modelType" => {
                    model_types.push(ModelType::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"decodedData" => break,
                Event::Eof => panic!("unexpected eof when parsing decodedData"),
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        panic!(
                            "unexpected text when parsing decodedData at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        )
                    }
                }
                unexpected => panic!(
                    "unexpected event when parsing decodedData at position {}: {:?}",
                    reader.buffer_position(),
                    unexpected
                )
            }
            buf.clear();
        }
        Ok(DecodedData {
            model_types
        })
    }
}


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
                Event::Eof => panic!("unexpected eof when parsing modelType"),
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        panic!(
                            "unexpected text when parsing modelType at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        )
                    }
                }
                unexpected => panic!(
                    "unexpected event when parsing modelType at position {}: {:?}",
                    reader.buffer_position(),
                    unexpected
                )
            }
        }
        let map = attributes_to_map(e)?;
        Ok(ModelType {
            dtype: map.get("type").cloned().ok_or("missing type")?,
            models,
        })
    }
}


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
                Event::Empty(e) if e.name().as_ref() == b"multiModelField" => {}
                Event::Empty(e) if e.name().as_ref() == b"multiField" => {
                    multi_fields.push(MultiField::parse_one_empty(&e)?);
                }
                Event::Start(e) if e.name().as_ref() == b"multiField" => {
                    multi_fields.push(MultiField::parse_one(&e, reader)?);
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
                Event::Eof => panic!("unexpected eof when parsing model"),
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        panic!(
                            "unexpected text when parsing model at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        )
                    }
                }
                unexpected => panic!(
                    "unexpected event when parsing model at position {}: {:?}",
                    reader.buffer_position(),
                    unexpected
                )
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
                Event::Eof => panic!("unexpected eof when parsing modelField"),
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        panic!(
                            "unexpected text when parsing modelField at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        )
                    }
                }
                unexpected => panic!(
                    "unexpected event when parsing modelField at position {}: {:?}",
                    reader.buffer_position(),
                    unexpected
                )
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




#[derive(Debug, PartialEq)]
pub struct MultiModelField {
    pub name: String,
    pub dtype: String,
    pub models: Vec<Model>
}

impl MultiModelField {
    pub fn parse_one<B: BufRead>(e: &BytesStart, reader: &mut Reader<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        let mut models = Vec::new();
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"model" => {
                    models.push(Model::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"multiModelField" => break,
                Event::Eof => panic!("unexpected eof when parsing multiModelField"),
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        panic!(
                            "unexpected text when parsing multiModelField at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        )
                    }
                }
                unexpected => panic!(
                    "unexpected event when parsing multiModelField at position {}: {:?}",
                    reader.buffer_position(),
                    unexpected
                )
            }
            buf.clear();
        }
        
        Ok(MultiModelField {
            name: map.get("name").cloned().ok_or("missing name")?,
            dtype: map.get("type").cloned().ok_or("missing type")?,
            models,
        })
    }
}



#[derive(Debug, PartialEq)]
pub struct MultiField {
    pub name: String,
    pub dtype: String,
}

impl MultiField {
    pub fn parse_one<B: BufRead>(e: &BytesStart, reader: &mut Reader<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Empty(e) if e.name().as_ref() == b"empty" => {}
                Event::End(e) if e.name().as_ref() == b"multiField" => break,
                Event::Eof => panic!("unexpected eof when parsing multiField"),
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        panic!(
                            "unexpected text when parsing multiField at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        )
                    }
                }
                unexpected => panic!(
                    "unexpected event when parsing multiField at position {}: {:?}",
                    reader.buffer_position(),
                    unexpected
                )
            }
            buf.clear();
        }
        Ok(MultiField {
            name: map.get("name").cloned().ok_or("missing name")?,
            dtype: map.get("type").cloned().ok_or("missing type")?,
        })
    }

        pub fn parse_one_empty(e: &BytesStart) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        Ok(MultiField {
            name: map.get("name").cloned().ok_or("missing name")?,
            dtype: map.get("type").cloned().ok_or("missing type")?,
        })
    }
}