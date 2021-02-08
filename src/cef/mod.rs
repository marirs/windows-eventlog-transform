#[macro_use]
pub(crate) mod macros;
mod properties;
mod parse;
mod cef_map;

use serde_json::to_string;

use crate::{Event, mappings, ToCEF};

impl ToCEF for Event {
    /// Convert the Event object to Common Event Format (CEF) String
    fn to_cef(&self) -> String {
        let system = self.System.clone();
        let event_data = if let Some(d) = self.EventData.clone() {
            parse::get_event_data(system.Event.EventID, d)
        } else { String::new() };
        // Add a message if message field is not there
        let event_data = if !event_data.contains("msg=") {
            format!("msg={} {}", system.Event.EventName.clone(), event_data)
        } else {
            event_data
        };

        let raw_str = to_string(self).unwrap();

        // Create the CEF Header
        let cef_header = format!(
            "CEF:0|Microsoft|Windows|{product_ver}|{clss_id}|{name}|{sev}|",
            product_ver = "Windows 7",
            clss_id=parse::get_class_id(self.clone()),
            name=system.Event.EventName,
            sev=mappings::levels::from_string_to_usize(system.Level)
        );

        // Start & Build the CEF extension
        let cef_extension = format!(
            "start={start} externalId={event_id} \
            outcome={outcome} \
            cn1Label=EventRecordId cn1={record_id} \
            cs1Label=Opcode cs1={opcode} \
            cs2Label=Keywords cs2={keywords}",
            start=system.TimeCreated,
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
        let cef_extension = format!("{} rawEvent={}", cef_extension, raw_str);

        // Return the CEF String
        cef_header + cef_extension.as_str()
    }
}
