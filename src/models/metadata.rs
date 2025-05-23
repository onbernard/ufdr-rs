use std::io::BufRead;
use quick_xml::{events::{BytesStart, Event}, Reader};
use super::{attributes_to_map, Item, ParseError};



#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub section: String,
    pub items: Vec<Item>
}

impl Metadata {
    pub fn parse_one<B: BufRead>(
        e: &BytesStart,
        reader: &mut Reader<B>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let map = attributes_to_map(e)?;
        let mut items = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"item" => {
                    items.push(Item::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"metadata" => break,
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing metadata")));
                },
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                    } else {
                        return Err(Box::new(ParseError::new(&format!(
                            "unexpected text when parsing metadata at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ))));
                    }
                }
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing metadata at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
            buf.clear();
        }

        Ok(Metadata {
            section: map.get("section").cloned().ok_or("missing section")?,
            items,
        })
    }
}



#[cfg(test)]
mod test {
    use std::io::Cursor;
    use quick_xml::{events::Event, Reader};
    use super::*;

    fn test_metadata(xml_str: &str, expected: Metadata) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"metadata" => {
                    let uwu = Metadata::parse_one(&e, &mut reader);
                    if let Ok(metadata) = uwu {
                        let known_keys = vec![
                            "section",
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown metadata attribute: {}", key);
                        }
                        assert_eq!(metadata, expected);
                        return Ok(());
                    } else {
                        return Err(format!("Metadata::parse_one error {:#?}", uwu));
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
    fn test_metadata_0() -> Result<(), String> {
        let xml_str = r#"
        <metadata section="Additional Fields">
            <item name="DeviceInfoCreationTime"><![CDATA[10/10/2020 13:37:00]]></item>
            <item name="UFED_PA_Version"><![CDATA[1.2.3.4]]></item>
        </metadata>
        "#;
        test_metadata(xml_str, Metadata {
            section: "Additional Fields".to_string(),
            items: vec![
                Item {
                    id: None,
                    name: "DeviceInfoCreationTime".to_string(),
                    group: None,
                    source_extraction: None,
                    text: "10/10/2020 13:37:00".to_string(),
                },
                Item {
                    id: None,
                    name: "UFED_PA_Version".to_string(),
                    group: None,
                    source_extraction: None,
                    text: "1.2.3.4".to_string(),
                },
            ]
        })
    }

    #[test]
    fn test_metadata_1() -> Result<(), String> {
        let xml_str = r#"
        <metadata section="Extraction Data">
            <item name="DeviceInfoSelectedManufacturer" sourceExtraction="0"><![CDATA[Apple]]></item>
            <item name="DeviceInfoConnectionType" sourceExtraction="0"><![CDATA[Apple]]></item>
            <item name="ExtractionType" sourceExtraction="0"><![CDATA[Logical]]></item>
            <item name="ProjectStateExtractionId" sourceExtraction="0"><![CDATA[12345678-a931-45a1-b3a1-1234567890A]]></item>
            <item name="DeviceInfoExtractionStartDateTime" sourceExtraction="0"><![CDATA[10/10/2020 13:37:00]]></item>
            <item name="DeviceInfoExtractionEndDateTime" sourceExtraction="0"><![CDATA[10/10/2020 14:00:00]]></item>
        </metadata>
        "#;
        test_metadata(xml_str, Metadata {
            section: "Extraction Data".to_string(),
            items: vec![
                Item {
                    id: None,
                    name: "DeviceInfoSelectedManufacturer".to_string(),
                    group: None,
                    source_extraction: Some("0".to_string()),
                    text: "Apple".to_string(),
                },
                Item {
                    id: None,
                    name: "DeviceInfoConnectionType".to_string(),
                    group: None,
                    source_extraction: Some("0".to_string()),
                    text: "Apple".to_string(),
                },
                Item {
                    id: None,
                    name: "ExtractionType".to_string(),
                    group: None,
                    source_extraction: Some("0".to_string()),
                    text: "Logical".to_string(),
                },
                Item {
                    id: None,
                    name: "ProjectStateExtractionId".to_string(),
                    group: None,
                    source_extraction: Some("0".to_string()),
                    text: "12345678-a931-45a1-b3a1-1234567890A".to_string(),
                },
                Item {
                    id: None,
                    name: "DeviceInfoExtractionStartDateTime".to_string(),
                    group: None,
                    source_extraction: Some("0".to_string()),
                    text: "10/10/2020 13:37:00".to_string(),
                },
                Item {
                    id: None,
                    name: "DeviceInfoExtractionEndDateTime".to_string(),
                    group: None,
                    source_extraction: Some("0".to_string()),
                    text: "10/10/2020 14:00:00".to_string(),
                },
            ]
        })
    }

        #[test]
    fn test_metadata_2() -> Result<(), String> {
        let xml_str = r#"
        <metadata section="Device Info">
            <item id="78885761-d2fd-4179-84eb-70b4a19b809c" name="DeviceInfoBaseBandVersion" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[1.2.31]]></item>
            <item id="dc8707a8-d227-4715-ba39-b59632543eb6" name="DeviceInfoOSVersion" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[1.2.3]]></item>
            <item id="0da47208-0b0d-46ad-9eb3-b3ebd9eb8f69" name="DeviceInfoActivationState" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[Activated]]></item>
            <item id="579d8e32-8255-4286-870a-93efecce33de" name="DeviceInfoBluetoothDeviceAddress" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[12:34:56:78:90:ab]]></item>
            <item id="e56c40e2-40af-4ffc-9197-58a4e0e64e58" name="DeviceInfoSimStatus" group="iPhone of Prof Moriarty" sourceExtraction=""><![CDATA[Ready]]></item>
            <item id="dbb39d9d-1288-4546-9e71-e7ac5e03b441" name="DeviceInfoWiFiAddress" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[12:34:56:78:90:ab]]></item>
            <item id="4db3ebf8-f9b7-4f7a-9912-74e4d26c140b" name="DeviceInfoStorageCapacity" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[42 GB]]></item>
            <item id="49381e39-0e49-469e-b1bf-751b0495f652" name="DeviceInfoStorageAvailable" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[21 GB]]></item>
            <item id="e0e74416-208e-4aaf-b558-f2156432e65f" name="DeviceInfoTimeZone" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[Europe/Vienna]]></item>
            <item id="d0c17d7a-7bdb-4c2d-85a3-6154b649879d" name="DeviceInfoModelNumber" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[ABCDEF]]></item>
            <item id="354c50b2-7757-43fe-9ca7-f8cba8c01e4d" name="DeviceInfoOSType" group="Metadata" sourceExtraction="0"><![CDATA[iOS]]></item>
            <item id="210ab633-3908-4c59-b0b9-da29ec4b1da6" name="DeviceInfoTimeZone" group="Phone Settings" sourceExtraction="0"><![CDATA[Europe/Vienna]]></item>
            <item id="806912da-1d09-476e-b3e6-5cdd5b5d6791" name="DeviceInfoPhoneDateTime" sourceExtraction="0"><![CDATA[09.09.2020 19:33:13(UTC+0)]]></item>
            <item id="92880fa1-d8ec-43ba-b3a3-c9d7824ae90a" name="DeviceInfoLocaleLanguage" group="Phone Settings" sourceExtraction="0"><![CDATA[de_AT]]></item>
            <item id="6bef83e8-bdd8-48ec-92f1-653122740fc1" name="DeviceInfoCloudBackupEnabled" group="Phone Settings" sourceExtraction="0"><![CDATA[True]]></item>
        </metadata>
        "#;
        test_metadata(xml_str, Metadata {
            section: "Device Info".to_string(),
            items: vec![
                Item {
                    id: Some("78885761-d2fd-4179-84eb-70b4a19b809c".to_string()),
                    name: "DeviceInfoBaseBandVersion".to_string(),
                    group: Some("iPhone of Prof Moriarty".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "1.2.31".to_string(),
                },
                Item {
                    id: Some("dc8707a8-d227-4715-ba39-b59632543eb6".to_string()),
                    name: "DeviceInfoOSVersion".to_string(),
                    group: Some("iPhone of Prof Moriarty".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "1.2.3".to_string(),
                },
                Item {
                    id: Some("0da47208-0b0d-46ad-9eb3-b3ebd9eb8f69".to_string()),
                    name: "DeviceInfoActivationState".to_string(),
                    group: Some("iPhone of Prof Moriarty".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "Activated".to_string(),
                },
                Item {
                    id: Some("579d8e32-8255-4286-870a-93efecce33de".to_string()),
                    name: "DeviceInfoBluetoothDeviceAddress".to_string(),
                    group: Some("iPhone of Prof Moriarty".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "12:34:56:78:90:ab".to_string(),
                },
                Item {
                    id: Some("e56c40e2-40af-4ffc-9197-58a4e0e64e58".to_string()),
                    name: "DeviceInfoSimStatus".to_string(),
                    group: Some("iPhone of Prof Moriarty".to_string()),
                    source_extraction: Some("".to_string()),
                    text: "Ready".to_string(),
                },
                Item {
                    id: Some("dbb39d9d-1288-4546-9e71-e7ac5e03b441".to_string()),
                    name: "DeviceInfoWiFiAddress".to_string(),
                    group: Some("iPhone of Prof Moriarty".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "12:34:56:78:90:ab".to_string(),
                },
                Item {
                    id: Some("4db3ebf8-f9b7-4f7a-9912-74e4d26c140b".to_string()),
                    name: "DeviceInfoStorageCapacity".to_string(),
                    group: Some("iPhone of Prof Moriarty".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "42 GB".to_string(),
                },
                Item {
                    id: Some("49381e39-0e49-469e-b1bf-751b0495f652".to_string()),
                    name: "DeviceInfoStorageAvailable".to_string(),
                    group: Some("iPhone of Prof Moriarty".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "21 GB".to_string(),
                },
                Item {
                    id: Some("e0e74416-208e-4aaf-b558-f2156432e65f".to_string()),
                    name: "DeviceInfoTimeZone".to_string(),
                    group: Some("iPhone of Prof Moriarty".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "Europe/Vienna".to_string(),
                },
                Item {
                    id: Some("d0c17d7a-7bdb-4c2d-85a3-6154b649879d".to_string()),
                    name: "DeviceInfoModelNumber".to_string(),
                    group: Some("iPhone of Prof Moriarty".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "ABCDEF".to_string(),
                },
                Item {
                    id: Some("354c50b2-7757-43fe-9ca7-f8cba8c01e4d".to_string()),
                    name: "DeviceInfoOSType".to_string(),
                    group: Some("Metadata".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "iOS".to_string(),
                },
                Item {
                    id: Some("210ab633-3908-4c59-b0b9-da29ec4b1da6".to_string()),
                    name: "DeviceInfoTimeZone".to_string(),
                    group: Some("Phone Settings".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "Europe/Vienna".to_string(),
                },
                Item {
                    id: Some("806912da-1d09-476e-b3e6-5cdd5b5d6791".to_string()),
                    name: "DeviceInfoPhoneDateTime".to_string(),
                    group: None,
                    source_extraction: Some("0".to_string()),
                    text: "09.09.2020 19:33:13(UTC+0)".to_string(),
                },
                Item {
                    id: Some("92880fa1-d8ec-43ba-b3a3-c9d7824ae90a".to_string()),
                    name: "DeviceInfoLocaleLanguage".to_string(),
                    group: Some("Phone Settings".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "de_AT".to_string(),
                },
                Item {
                    id: Some("6bef83e8-bdd8-48ec-92f1-653122740fc1".to_string()),
                    name: "DeviceInfoCloudBackupEnabled".to_string(),
                    group: Some("Phone Settings".to_string()),
                    source_extraction: Some("0".to_string()),
                    text: "True".to_string(),
                },
            ]
        })
    }
}