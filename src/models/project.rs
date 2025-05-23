use std::io::BufRead;
use quick_xml::{events::{BytesStart, Event}, Reader};
use super::{attributes_to_map, ParseError, CaseInformation, DecodedData, Images, Metadata, SourceExtractions, TaggedFiles};



#[derive(Debug, PartialEq)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub report_version: String,
    pub license_id: String,
    pub contains_garbage: String,
    pub extraction_type: String,
    pub node_count: String,
    pub model_count: String,
    pub xmlns: String,
    pub source_extractions: SourceExtractions,
    pub case_information: CaseInformation,
    pub metadata: Vec<Metadata>,
    pub images: Option<Images>,
    pub tagged_files: Option<TaggedFiles>,
    pub decoded_data: Option<DecodedData>,
}

impl Project {
    pub fn parse_one<B: BufRead>(
        e: &BytesStart,
        reader: &mut Reader<B>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let map = attributes_to_map(e)?;
        let mut source_extractions: Option<SourceExtractions> = None;
        let mut case_information = None;
        let mut metadata = vec![];
        let mut images = None;
        let mut tagged_files = None;
        let mut decoded_data = None;
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"sourceExtractions" => {
                    source_extractions = Some(SourceExtractions::parse_one(reader)?);
                }
                Event::Start(e) if e.name().as_ref() == b"caseInformation" => {
                    case_information = Some(CaseInformation::parse_one(reader)?);
                }
                Event::Start(e) if e.name().as_ref() == b"metadata" => {
                    metadata.push(Metadata::parse_one(&e, reader)?);
                }
                Event::Start(e) if e.name().as_ref() == b"images" => {
                    images = Some(Images::parse_one(reader)?);
                }
                Event::Start(e) if e.name().as_ref() == b"taggedFiles" => {
                    tagged_files = Some(TaggedFiles::parse_one(reader)?);
                }
                Event::Start(e) if e.name().as_ref() == b"decodedData" => {
                    decoded_data = Some(DecodedData::parse_one(reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"project" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing project")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing project at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing project at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
            buf.clear();
        }
        Ok(Project {
            id: map.get("id").cloned().ok_or("missing id")?,
            name: map.get("name").cloned().ok_or("missing name")?,
            report_version: map.get("reportVersion").cloned().ok_or("missing reportVersion")?,
            license_id: map.get("licenseID").cloned().ok_or("missing licenseID")?,
            contains_garbage: map.get("containsGarbage").cloned().ok_or("missing containsGarbage")?,
            extraction_type: map.get("extractionType").cloned().ok_or("missing extractionType")?,
            node_count: map.get("NodeCount").cloned().ok_or("missing NodeCount")?,
            model_count: map.get("ModelCount").cloned().ok_or("missing ModelCount")?,
            xmlns: map.get("xmlns").cloned().ok_or("missing xmlns")?,
            source_extractions: source_extractions.ok_or("missing sourceExtractions")?,
            case_information: case_information.ok_or("missing caseInformation")?,
            metadata,
            images,
            tagged_files,
            decoded_data,
        })
    }
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader};
    use super::*;

    #[test]
    fn test_project_0() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("test_data/xml/report.xml")?;
        let mut reader = Reader::from_reader(BufReader::new(file));
        let mut buf = Vec::new();
        loop {
            let event = reader
                .read_event_into(&mut buf)?;
            match event {
                Event::Start(ref e) if e.name().as_ref() == b"project" => {
                    let proj = Project::parse_one(e, &mut reader)?;
                    println!("{:#?}", proj);
                    break;
                }
    
                Event::Eof => break,
    
                _ => {}
            }
    
            buf.clear();
        }
        Ok(())
    }
}