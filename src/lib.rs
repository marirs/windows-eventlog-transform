mod de;
use de::Event;

mod cef;
pub use cef::ToCEF;

mod mappers;

use quick_xml::de::from_str;

use std::{
    io::Read,
    fs::File,
    path::Path,
};

/// Deserialize Windows Event Log XML from a file
///
/// ## Example usage
/// ```rust
/// let e = winevents_xml_transform::from_file("data/winevt1.xml").unwrap();
/// println!("{:#?}", e);
/// ```
pub fn from_file<P: AsRef<Path>>(xml_path: P) -> Result<Event, String> {
    if !Path::new(xml_path.as_ref()).exists() {
        return Err(format!("{} not found", xml_path.as_ref().to_str().unwrap()))
    }

    let mut data = String::new();
    {
        let mut f = File::open(xml_path).unwrap();
        f.read_to_string(&mut data).unwrap();
    }

    from_string(data)
}

/// Deserialize Windows Event Log XML from string
///
/// ## Example usage
/// ```rust
/// let xml_string = r#"<?xml version="1.0" encoding="utf-8"?>
/// <Event xmlns="http://schemas.microsoft.com/win/2004/08/events/event">
///     <System>
///         <Provider Name="Microsoft-Windows-Security-Auditing" Guid="{54849625-5478-4994-a5ba-3e3b0328c30d}" />
///         <EventID>4624</EventID>
///         <Version>2</Version>
///         <Level>0</Level>
///         <Task>12544</Task>
///         <Opcode>0</Opcode>
///         <Keywords>0x8020000000000000</Keywords>
///         <TimeCreated SystemTime="2021-01-26T11:17:29.4856969Z" />
///         <EventRecordID>26893</EventRecordID>
///         <Correlation ActivityID="{1f813878-e986-0000-f838-811f86e9d601}" />
///         <Execution ProcessID="648" ThreadID="2368" />
///         <Channel>Security</Channel>
///         <Computer>DESKTOP-G089JUF</Computer>
///         <Security />
///     </System>
///     <EventData>
///         <Data Name="SubjectUserSid">S-1-5-18</Data>
///         <Data Name="SubjectUserName">DESKTOP-G089JUF$</Data>
///         <Data Name="SubjectDomainName">WORKGROUP</Data>
///     </EventData>
/// </Event>"#;
/// let e = winevents_xml_transform::from_string(xml_string.to_string()).unwrap();
/// println!("{:#?}", e);
/// ```
pub fn from_string(xml_string: String) -> Result<Event, String> {
    let result: Event = match from_str(&xml_string) {
        Ok(res) => res,
        Err(e) => {
            return Err(format!("{}", e))
        }
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{from_file, from_string};

    const XML_STRING: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <Event xmlns="http://schemas.microsoft.com/win/2004/08/events/event">
            <System>
                <Provider Name="Microsoft-Windows-Security-Auditing" Guid="{54849625-5478-4994-a5ba-3e3b0328c30d}" />
                <EventID>4624</EventID>
                <Version>2</Version>
                <Level>0</Level>
                <Task>12544</Task>
                <Opcode>0</Opcode>
                <Keywords>0x8020000000000000</Keywords>
                <TimeCreated SystemTime="2021-01-26T11:17:29.4856969Z" />
                <EventRecordID>26893</EventRecordID>
                <Correlation ActivityID="{1f813878-e986-0000-f838-811f86e9d601}" />
                <Execution ProcessID="648" ThreadID="2368" />
                <Channel>Security</Channel>
                <Computer>DESKTOP-G089JUF</Computer>
                <Security />
            </System>
            <EventData>
                <Data Name="SubjectUserSid">S-1-5-18</Data>
                <Data Name="SubjectUserName">DESKTOP-G089JUF$</Data>
                <Data Name="SubjectDomainName">WORKGROUP</Data>
            </EventData>
        </Event>"#;

    #[test]
    fn test_from_file() {
        let e = from_file("data/winevt1.xml");
        assert!(e.is_ok());
    }

    #[test]
    fn test_from_xml_string() {
        let e = from_string(XML_STRING.to_string());
        assert!(e.is_ok());
    }

    #[test]
    fn test_time_created_flatten() {
        let e = from_string(XML_STRING.to_string());
        let tc = e.unwrap();
        let tc = tc.System.TimeCreated;
        assert_eq!(tc, "2021-01-26T11:17:29.4856969Z")
    }

    #[test]
    fn test_xmlns() {
        let e = from_string(XML_STRING.to_string());
        let xmlns = e.unwrap();
        assert_eq!(xmlns.xmlns, "http://schemas.microsoft.com/win/2004/08/events/event")
    }
}