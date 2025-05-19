use pyo3::{prelude::*,exceptions::PyIOError};
use quick_xml::{events::Event, Reader};
use std::{fs::File, io::BufReader, str};


pub mod utils;
pub mod models;

use models::{SourceExtractions, CaseInformation, Metadata, Images, TaggedFiles, DecodedData};


#[pyfunction]
fn xml(path: &str) -> PyResult<Option<DecodedData>> {
    let file = File::open(path).map_err(|e| PyIOError::new_err(e.to_string()))?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    let mut buf = Vec::new();

    loop {
        let event = reader
            .read_event_into(&mut buf)
            .map_err(|e| PyIOError::new_err(e.to_string()))?;

        match event {
            Event::Start(ref e) if e.name().as_ref() == b"sourceExtractions" => {
                let source_extractions = SourceExtractions::parse_one(&mut reader).map_err(|e| PyIOError::new_err(e.to_string()))?;
                println!("{:#?}", source_extractions);
            }

            Event::Start(ref e) if e.name().as_ref() == b"caseInformation" => {
                let case_information = CaseInformation::parse_one(&mut reader).map_err(|e| PyIOError::new_err(e.to_string()))?;
                // println!("{:#?}", case_information);
            }

            Event::Start(ref e) if e.name().as_ref() == b"metadata" => {
                let metadata = Metadata::parse_one(e, &mut reader).map_err(|e| PyIOError::new_err(e.to_string()))?;
                // println!("{:#?}", metadata);
            }

            Event::Start(ref e) if e.name().as_ref() == b"images" => {
                let images = Images::parse_one(&mut reader).map_err(|e| PyIOError::new_err(e.to_string()))?;
                // println!("{:#?}", images);
            }

            Event::Start(ref e) if e.name().as_ref() == b"taggedFiles" => {
                let tagged_files = TaggedFiles::parse_one(&mut reader).map_err(|e| PyIOError::new_err(e.to_string()))?;
                // println!("{:#?}", tagged_files);
            }

            Event::Start(ref e) if e.name().as_ref() == b"decodedData" => {
                let decoded_data = DecodedData::parse_one(&mut reader).map_err(|e| PyIOError::new_err(e.to_string()))?;
                // return Ok(Some(decoded_data));
                // println!("{:#?}", decoded_data);
            }

            Event::Eof => break,

            _ => {}
        }

        buf.clear();
    }

    Ok(None)
}



#[pyfunction]
fn hello_from_bin() -> String {
    "Hello from ufdr!".to_string()
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    m.add_function(wrap_pyfunction!(xml, m)?)?;
    Ok(())
}
