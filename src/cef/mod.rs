use serde_json;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod parser;
pub(crate) mod utils;

use crate::{
    Event,
    mappers::levels,
};

pub trait ToCEF {
    fn to_cef(&self) -> String;
}

impl ToCEF for Event {
    fn to_cef(&self) -> String {
        //! Converts Windows Event into
        //! Common Event Format (CEF)

        // save the raw event
        let cef_raw = serde_json::to_string(self).unwrap();

        // Build the CEF header
        let cef_header = format!(
            "CEF:0|Microsoft|Windows|{product_ver}|{clss_id}|{name}|{sev}|",
            product_ver = "Windows 7",
            clss_id=parser::get_class_id(self),
            name=self.System.Event.EventName,
            sev=levels::from_string_to_usize(&self.System.Level)
        );

        // Build the CEF extension
        let cef_extension = parser::build_cef_extension(&self);
        let cef_extension = cef_extension
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join(" ");

        // final cef string
        format!(
            "{}{} rawEvent={}",
            cef_header.replace("\"", ""),
            cef_extension.replace("\"", ""),
            cef_raw
        )
    }
}