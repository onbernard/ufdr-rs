use models::Project;
use quick_xml::{events::Event, Reader};
use std::{fs::File, io::BufReader};

mod models;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("data/xml/report.xml")?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    let mut buf = Vec::new();
    loop {
        let event = reader.read_event_into(&mut buf)?;
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
