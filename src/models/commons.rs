use std::io::BufRead;
use quick_xml::{events::{BytesStart, Event}, Reader};
use crate::utils::{attributes_to_map, read_text, ParseError};



#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub is_system: Option<String>,
    pub is_required: Option<String>,
    pub field_type: Option<String>,
    pub multiple_lines: Option<String>,
    pub dtype: Option<String>,
    pub text: String,
    pub value: Option<Value>,
}

impl Field {
    pub fn parse_one<B: BufRead>(e: &BytesStart, reader: &mut Reader<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        let mut buf = Vec::new();
        let mut text = String::new();
        let mut value = None;
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Empty(e) if e.name().as_ref() == b"empty" => {}
                Event::Start(e) if e.name().as_ref() == b"value" => {
                    value = Some(Value::parse_one(&e, reader)?);
                }
                Event::End(e) if e.name().as_ref() == b"field" => break,
                Event::Text(e) => {
                    text.push_str(&e.unescape()?.to_string());
                }
                Event::CData(e) => {
                    text.push_str(std::str::from_utf8(&e)?.trim());
                }
                Event::Eof => {
                    return Err(Box::new(ParseError::new("unexpected EOF when parsing field")));
                },
                unexpected => {
                    return Err(Box::new(ParseError::new(&format!(
                        "unexpected event when parsing field at position {}: {:?}",
                        reader.buffer_position(),
                        unexpected
                    ))));
                }
            }
            buf.clear();
        }
        Ok(Field {
            name: map.get("name").cloned().ok_or("missing name")?,
            is_system: map.get("isSystem").cloned(),
            is_required: map.get("isRequired").cloned(),
            field_type: map.get("fieldType").cloned(),
            multiple_lines: map.get("multipleLines").cloned(),
            dtype: map.get("type").cloned(),
            text: text.chars().filter(|c| !c.is_whitespace()).collect(),
            value,
        })
    }
}


#[derive(Debug, PartialEq)]
pub struct Item {
    pub id: Option<String>,
    pub name: String,
    pub group: Option<String>,
    pub source_extraction: Option<String>,
    pub text: String,
}

impl Item {
    pub fn parse_one<B: BufRead>(e: &BytesStart, reader: &mut Reader<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        let text = read_text(reader)?;
        Ok(Item {
            id: map.get("id").cloned(),
            name: map.get("name").cloned().ok_or("missing name")?,
            group: map.get("group").cloned(),
            source_extraction: map.get("sourceExtraction").cloned(),
            text,
        })
    }
}


#[derive(Debug, PartialEq)]
pub struct Value {
    pub dtype: String,
    pub text: String,
}

impl Value {
    pub fn parse_one<B: BufRead>(e: &BytesStart, reader: &mut Reader<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let map = attributes_to_map(e)?;
        let text = read_text(reader)?;
        Ok(Value {
            dtype: map.get("type").cloned().ok_or("missing type")?,
            text,
        })
    }
}


#[cfg(test)]
mod test {
    use std::io::Cursor;
    use quick_xml::{events::Event, Reader};
    use super::*;

    fn test_field(xml_str: &str, expected: Field) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"field" => {
                    if let Ok(field) = Field::parse_one(&e, &mut reader) {
                        assert_eq!(field, expected);
                        return Ok(());
                    } else {
                        return Err("Field::parse_one error".to_string());
                    }
                }
                Ok(Event::Eof) => {
                    return Err("eof".to_string());
                }
                a => {
                    // println!("{:#?}", a);
                },
            }
        }
    }

    #[test]
    fn test_field_0() -> Result<(), String> {
        let xml_str = r#"
        <field name="Fall-Nummer" isSystem="True" isRequired="False" fieldType="CaseNumber" multipleLines="False">Case 001</field>
        "#;
        test_field(xml_str, Field {
            name: "Fall-Nummer".to_string(),
            is_system: Some("True".to_string()),
            is_required: Some("False".to_string()),
            field_type: Some("CaseNumber".to_string()),
            multiple_lines: Some("False".to_string()),
            dtype: None,
            text: "Case 001".to_string(),
            value: None,
        })
    }

    #[test]
    fn test_field_1() -> Result<(), String> {
        let xml_str = r#"
        <field name="Fallname" isSystem="True" isRequired="False" fieldType="CaseName" multipleLines="False">Super important case</field>
        "#;
        test_field(xml_str, Field {
            name: "Fallname".to_string(),
            is_system: Some("True".to_string()),
            is_required: Some("False".to_string()),
            field_type: Some("CaseName".to_string()),
            multiple_lines: Some("False".to_string()),
            dtype: None,
            text: "Super important case".to_string(),
            value: None,
        })
    }

    #[test]
    fn test_field_2() -> Result<(), String> {
        let xml_str = r#"
        <field name="Beweisnummer" isSystem="True" isRequired="False" fieldType="EvidenceNumber" multipleLines="False">001</field>
        "#;
        test_field(xml_str, Field {
            name: "Beweisnummer".to_string(),
            is_system: Some("True".to_string()),
            is_required: Some("False".to_string()),
            field_type: Some("EvidenceNumber".to_string()),
            multiple_lines: Some("False".to_string()),
            dtype: None,
            text: "001".to_string(),
            value: None,
        })
    }

    #[test]
    fn test_field_3() -> Result<(), String> {
        let xml_str = r#"
        <field name="Name d. Ermittlers" isSystem="False" isRequired="False" fieldType="None" multipleLines="False">Sherlock Holmes</field>
        "#;
        test_field(xml_str, Field {
            name: "Name d. Ermittlers".to_string(),
            is_system: Some("False".to_string()),
            is_required: Some("False".to_string()),
            field_type: Some("None".to_string()),
            multiple_lines: Some("False".to_string()),
            dtype: None,
            text: "Sherlock Holmes".to_string(),
            value: None,
        })
    }

    #[test]
    fn test_field_4() -> Result<(), String> {
        let xml_str = r#"
        <field name="Abteilung" isSystem="False" isRequired="False" fieldType="None" multipleLines="False">Department of Investigation</field>
        "#;
        test_field(xml_str, Field {
            name: "Abteilung".to_string(),
            is_system: Some("False".to_string()),
            is_required: Some("False".to_string()),
            field_type: Some("None".to_string()),
            multiple_lines: Some("False".to_string()),
            dtype: None,
            text: "Department of Investigation".to_string(),
            value: None,
        })
    }

    #[test]
    fn test_field_5() -> Result<(), String> {
        let xml_str = r#"
        <field name="Ort" isSystem="False" isRequired="False" fieldType="None" multipleLines="False">Vienna</field>
        "#;
        test_field(xml_str, Field {
            name: "Ort".to_string(),
            is_system: Some("False".to_string()),
            is_required: Some("False".to_string()),
            field_type: Some("None".to_string()),
            multiple_lines: Some("False".to_string()),
            dtype: None,
            text: "Vienna".to_string(),
            value: None,
        })
    }

    #[test]
    fn test_field_6() -> Result<(), String> {
        let xml_str = r#"
        <field name="Beschuldigter" isSystem="False" isRequired="False" fieldType="None" multipleLines="False">Professor James Moriarty</field>
        "#;
        test_field(xml_str, Field {
            name: "Beschuldigter".to_string(),
            is_system: Some("False".to_string()),
            is_required: Some("False".to_string()),
            field_type: Some("None".to_string()),
            multiple_lines: Some("False".to_string()),
            dtype: None,
            text: "Professor James Moriarty".to_string(),
            value: None,
        })
    }

    #[test]
    fn test_field_7() -> Result<(), String> {
        let xml_str = r#"
        <field name="PIN" isSystem="False" isRequired="False" fieldType="None" multipleLines="False">1337</field>
        "#;
        test_field(xml_str, Field {
            name: "PIN".to_string(),
            is_system: Some("False".to_string()),
            is_required: Some("False".to_string()),
            field_type: Some("None".to_string()),
            multiple_lines: Some("False".to_string()),
            dtype: None,
            text: "1337".to_string(),
            value: None,
        })
    }

    #[test]
    fn test_field_8() -> Result<(), String> {
        let xml_str = r#"
        <field name="UserMapping" type="Boolean">
            <value type="Boolean"><![CDATA[False]]></value>
        </field>
        "#;
        test_field(xml_str, Field {
            name: "UserMapping".to_string(),
            is_system: None,
            is_required: None,
            field_type: None,
            multiple_lines: None,
            dtype: Some("Boolean".to_string()),
            text: "".to_string(),
            value: Some(Value { dtype: "Boolean".to_string(), text: "False".to_string() }),
        })
    }

    #[test]
    fn test_field_9() -> Result<(), String> {
        let xml_str = r#"
        <field name="Id" type="String">
            <empty />
        </field>
        "#;
        test_field(xml_str, Field {
            name: "Id".to_string(),
            is_system: None,
            is_required: None,
            field_type: None,
            multiple_lines: None,
            dtype: Some("String".to_string()),
            text: "".to_string(),
            value: None,
        })
    }

    fn test_item(xml_str: &str, expected: Item) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"item" => {
                    if let Ok(item) = Item::parse_one(&e, &mut reader) {
                        assert_eq!(item, expected);
                        let known_keys = vec![
                            "id",
                            "name",
                            "group",
                            "sourceExtraction"
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown item attribute: {}", key);
                        }
                        return Ok(());
                    } else {
                        return Err("Item::parse_one error".to_string());
                    }
                }
                Ok(Event::Eof) => {
                    return Err("eof".to_string());
                }
                a => {
                    // println!("{:#?}", a);
                },
            }
        }
    }

    #[test]
    fn test_item_0() -> Result<(), String> {
        let xml_str = r#"
        <item name="DeviceInfoCreationTime"><![CDATA[10/10/2020 13:37:00]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "DeviceInfoCreationTime".to_string(),
            group: None,
            source_extraction: None,
            text: "10/10/2020 13:37:00".to_string(),
        })
    }

    #[test]
    fn test_item_1() -> Result<(), String> {
        let xml_str = r#"
        <item name="UFED_PA_Version"><![CDATA[1.2.3.4]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "UFED_PA_Version".to_string(),
            group: None,
            source_extraction: None,
            text: "1.2.3.4".to_string(),
        })
    }

    #[test]
    fn test_item_2() -> Result<(), String> {
        let xml_str = r#"
        <item name="DeviceInfoSelectedManufacturer" sourceExtraction="0"><![CDATA[Apple]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "DeviceInfoSelectedManufacturer".to_string(),
            group: None,
            source_extraction: Some("0".to_string()),
            text: "Apple".to_string(),
        })
    }

    #[test]
    fn test_item_3() -> Result<(), String> {
        let xml_str = r#"
        <item name="DeviceInfoConnectionType" sourceExtraction="0"><![CDATA[Cable No. 220]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "DeviceInfoConnectionType".to_string(),
            group: None,
            source_extraction: Some("0".to_string()),
            text: "Cable No. 220".to_string(),
        })
    }

    #[test]
    fn test_item_4() -> Result<(), String> {
        let xml_str = r#"
        <item name="ExtractionType" sourceExtraction="0"><![CDATA[Logical]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "ExtractionType".to_string(),
            group: None,
            source_extraction: Some("0".to_string()),
            text: "Logical".to_string(),
        })
    }

    #[test]
    fn test_item_6() -> Result<(), String> {
        let xml_str = r#"
        <item name="ProjectStateExtractionId" sourceExtraction="0"><![CDATA[12345678-a931-45a1-b3a1-1234567890A]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "ProjectStateExtractionId".to_string(),
            group: None,
            source_extraction: Some("0".to_string()),
            text: "12345678-a931-45a1-b3a1-1234567890A".to_string(),
        })
    }

    #[test]
    fn test_item_7() -> Result<(), String> {
        let xml_str = r#"
        <item name="DeviceInfoExtractionStartDateTime" sourceExtraction="0"><![CDATA[10/10/2020 13:37:00]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "DeviceInfoExtractionStartDateTime".to_string(),
            group: None,
            source_extraction: Some("0".to_string()),
            text: "10/10/2020 13:37:00".to_string(),
        })
    }

    #[test]
    fn test_item_8() -> Result<(), String> {
        let xml_str = r#"
        <item name="DeviceInfoExtractionEndDateTime" sourceExtraction="0"><![CDATA[10/10/2020 14:00:00]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "DeviceInfoExtractionEndDateTime".to_string(),
            group: None,
            source_extraction: Some("0".to_string()),
            text: "10/10/2020 14:00:00".to_string(),
        })
    }

    #[test]
    fn test_item_9() -> Result<(), String> {
        let xml_str = r#"
        <item id="78885761-d2fd-4179-84eb-70b4a19b809c" name="DeviceInfoBaseBandVersion" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[1.2.31]]></item>
        "#;
        test_item(xml_str, Item {
            id: Some("78885761-d2fd-4179-84eb-70b4a19b809c".to_string()),
            name: "DeviceInfoBaseBandVersion".to_string(),
            group: Some("iPhone of Prof Moriarty".to_string()),
            source_extraction: Some("0".to_string()),
            text: "1.2.31".to_string(),
        })
    }

    #[test]
    fn test_item_10() -> Result<(), String> {
        let xml_str = r#"
        <item id="dc8707a8-d227-4715-ba39-b59632543eb6" name="DeviceInfoOSVersion" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[1.2.3]]></item>
        "#;
        test_item(xml_str, Item {
            id: Some("dc8707a8-d227-4715-ba39-b59632543eb6".to_string()),
            name: "DeviceInfoOSVersion".to_string(),
            group: Some("iPhone of Prof Moriarty".to_string()),
            source_extraction: Some("0".to_string()),
            text: "1.2.3".to_string(),
        })
    }

    #[test]
    fn test_item_11() -> Result<(), String> {
        let xml_str = r#"
        <item id="0da47208-0b0d-46ad-9eb3-b3ebd9eb8f69" name="DeviceInfoActivationState" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[Activated]]></item>
        "#;
        test_item(xml_str, Item {
            id: Some("0da47208-0b0d-46ad-9eb3-b3ebd9eb8f69".to_string()),
            name: "DeviceInfoActivationState".to_string(),
            group: Some("iPhone of Prof Moriarty".to_string()),
            source_extraction: Some("0".to_string()),
            text: "Activated".to_string(),
        })
    }

    #[test]
    fn test_item_12() -> Result<(), String> {
        let xml_str = r#"
        <item id="579d8e32-8255-4286-870a-93efecce33de" name="DeviceInfoBluetoothDeviceAddress" group="iPhone of Prof Moriarty" sourceExtraction="0"><![CDATA[12:34:56:78:90:ab]]></item>
        "#;
        test_item(xml_str, Item {
            id: Some("579d8e32-8255-4286-870a-93efecce33de".to_string()),
            name: "DeviceInfoBluetoothDeviceAddress".to_string(),
            group: Some("iPhone of Prof Moriarty".to_string()),
            source_extraction: Some("0".to_string()),
            text: "12:34:56:78:90:ab".to_string(),
        })
    }

    #[test]
    fn test_item_13() -> Result<(), String> {
        let xml_str = r#"
        <item id="e56c40e2-40af-4ffc-9197-58a4e0e64e58" name="DeviceInfoSimStatus" group="iPhone of Prof Moriarty" sourceExtraction=""><![CDATA[Ready]]></item>
        "#;
        test_item(xml_str, Item {
            id: Some("e56c40e2-40af-4ffc-9197-58a4e0e64e58".to_string()),
            name: "DeviceInfoSimStatus".to_string(),
            group: Some("iPhone of Prof Moriarty".to_string()),
            source_extraction: Some("".to_string()),
            text: "Ready".to_string(),
        })
    }

    #[test]
    fn test_item_14() -> Result<(), String> {
        let xml_str = r#"
        <item name="Local Path"><![CDATA[files\Audio\En-Creative-Commons.ogg]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "Local Path".to_string(),
            group: None,
            source_extraction: None,
            text: r"files\Audio\En-Creative-Commons.ogg".to_string(),
        })
    }

    #[test]
    fn test_item_15() -> Result<(), String> {
        let xml_str = r#"
        <item name="SHA256"><![CDATA[]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "SHA256".to_string(),
            group: None,
            source_extraction: None,
            text: r"".to_string(),
        })
    }

    #[test]
    fn test_item_16() -> Result<(), String> {
        let xml_str = r#"
        <item name="MD5"><![CDATA[3d7f880de7e11d0940558da7dc7e709f]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "MD5".to_string(),
            group: None,
            source_extraction: None,
            text: r"3d7f880de7e11d0940558da7dc7e709f".to_string(),
        })
    }

    #[test]
    fn test_item_17() -> Result<(), String> {
        let xml_str = r#"
        <item name="Tags"><![CDATA[Audio]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "Tags".to_string(),
            group: None,
            source_extraction: None,
            text: r"Audio".to_string(),
        })
    }

    #[test]
    fn test_item_18() -> Result<(), String> {
        let xml_str = r#"
        <item name="iPhone-Domain"><![CDATA[AppDomain-com.some-audio-app]]></item>
        "#;
        test_item(xml_str, Item {
            id: None,
            name: "iPhone-Domain".to_string(),
            group: None,
            source_extraction: None,
            text: r"AppDomain-com.some-audio-app".to_string(),
        })
    }

    fn test_value(xml_str: &str, expected: Value) -> Result<(), String> {
        let mut reader = Reader::from_reader(Cursor::new(xml_str));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"value" => {
                    let uwu = Value::parse_one(&e, &mut reader);
                    if let Ok(image) = uwu {
                        let known_keys = vec![
                            "type",
                        ];
                        for key in attributes_to_map(&e).unwrap().keys() {
                            assert!(known_keys.contains(&key.as_ref()), "Unknown value attribute: {}", key);
                        }
                        assert_eq!(image, expected);
                        return Ok(());
                    } else {
                        return Err(format!("Value::parse_one error {:#?}", uwu));
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
    fn test_value_0() -> Result<(), String> {
        let xml_str = r#"
        <value type="Boolean"><![CDATA[False]]></value>
        "#;
        test_value(xml_str, Value {
            dtype: "Boolean".to_string(),
            text: "False".to_string()
        })
    }


    #[test]
    fn test_value_1() -> Result<(), String> {
        let xml_str = r#"
        <value type="TimeStamp">2020-07-01T07:45:53.000+00:00</value>
        "#;
        test_value(xml_str, Value {
            dtype: "TimeStamp".to_string(),
            text: "2020-07-01T07:45:53.000+00:00".to_string()
        })
    }
}