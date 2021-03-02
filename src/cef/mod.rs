use chrono::DateTime;
use serde_json::to_string;

use crate::Event;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod mappers;
mod properties;
mod parse;

pub trait ToCEF {
    fn to_cef(&self) -> String;
}

impl ToCEF for Event {
    /// Convert the Event object to Common Event Format (CEF) String
    fn to_cef(&self) -> String {
        let system = self.System.clone();
        // Parse the Event data into valid cef k=v mapping based upon
        // the windows event id's
        let event_data = if let Some(d) = self.EventData.clone() {
            parse::get_event_data(system.Event.EventID, d)
        } else { String::new() };
        // Typically based on a given event id, there should be a msg= field,
        // in-case the msg field is missing in the mapped event_data k=v mapping
        // then add the name as the msg field along with the event_data mappings
        let event_data = if !event_data.contains("msg=") {
            format!("msg={} {}", system.Event.EventName.clone(), event_data)
        } else {
            event_data
        };

        let raw_event_string = to_string(self).unwrap();

        // Create the CEF Header
        let cef_header = format!(
            "CEF:0|Microsoft|Windows|{product_ver}|{clss_id}|{name}|{sev}|",
            product_ver = "Windows 7",
            clss_id=parse::get_class_id(self.clone()),
            name=system.Event.EventName,
            sev= mappers::levels::from_string_to_usize(system.Level)
        );
        let start = DateTime::parse_from_rfc3339(system.TimeCreated.as_str());
        let start = if let Ok(e) = start {
            e.timestamp().to_string()
        } else {
            system.TimeCreated.to_string()
        };

        // Start & Build the CEF extension
        let cef_extension = format!(
            "start={start} externalId={event_id} \
            outcome={outcome} \
            cn1Label=EventRecordId cn1={record_id} \
            cs1Label=Opcode cs1={opcode} \
            cs2Label=Keywords cs2={keywords}",
            start=start,
            event_id=system.Event.EventID,
            record_id=system.EventRecordID,
            opcode=system.Opcode,
            keywords=system.Keywords,
            outcome=parse::get_event_outcome(system.Keywords.clone())
        );

        // Add the event data if available
        let cef_extension = if !event_data.is_empty() {
            format!("{} {}", cef_extension, event_data)
        } else {
            format!("{}", cef_extension)
        };

        // Add the raw json event
        let cef_extension = format!("{} rawEvent={}", cef_extension, raw_event_string);

        // Return the CEF String
        cef_header + cef_extension.as_str()
    }
}
