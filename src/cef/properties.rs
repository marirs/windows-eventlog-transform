use std::collections::HashMap;

use crate::cef::mappers::{
    cef_map::CefMap,
    load_mapping, EventMappingGetters,
};

type EventData = HashMap<String, String>;
type CefObject = HashMap<String, String>;

const EMPTY_STRING: String = String::new();

/// Map & convert the Events into a CEF HashMap
pub fn mapper(event_id: &usize, evt: &EventData) -> Option<CefObject> {
    let evt_mapping = load_mapping();
    let map = if let Some(m) = evt_mapping.get_mapping_info(&event_id) {
        // Found mapping information
        m
    }else {
        // We dont have a mapping, so return new()
        return None
    };

    // Load the CEF Mapping fields
    let cef_map = CefMap::load_cef_map();
    // Build the CEF HashMap Object
    let result = map
        .iter()
        .map(|(k, v)|{
            let xml_key = v;
            if k.eq_ignore_ascii_case("message") {
                // We have a message
                if xml_key.starts_with("concatenate") {
                    // We have a message that needs to be concatenated
                    format!("msg={}", do_msg_concat(xml_key, &evt))
                } else if xml_key.eq_ignore_ascii_case("all_of_data") {
                    // All_Of_Data found for message
                    format!("msg={}", do_all_of_data(&evt))
                } else {
                    // Normal message
                    format!("msg={}", xml_key)
                }
            } else if k.eq_ignore_ascii_case("device action"){
                    // Got a Device Action Message

            } else {
                // Mapping if not a message field
                let cef_key = cef_map.get_cef_field_or_default(&k);
                let cef_val = if xml_key.starts_with("one_of(") {
                    // One_Of(..) value encountered
                    do_one_of(xml_key, &evt)
                } else if xml_key.starts_with("all_of(") {
                    // One_Of(..) value encountered
                    do_all_of(xml_key, &evt)
                } else if xml_key.starts_with("both(") {
                    // Both(..) value encountered
                    xml_key
                        .split(",")
                        .map(|x| x.replace("both (", "").replace(")", ""))
                        .map(|x| {
                            evt.get(&x).unwrap_or(&EMPTY_STRING).to_string()
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                } else if xml_key.starts_with("concatenate") {
                    // Concatenate(..) values encountered
                    String::new()
                    // TODO: need to do the concat logic
                } else if xml_key.contains("IPv6 Address)") {
                    // IPv6 address mostly available
                    let xml_fields = xml_key
                        .splitn(2, " ")
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .iter()
                        .nth(0)
                        .unwrap_or(&EMPTY_STRING)
                        .to_string();
                    let addr = if let Some(a) = evt.get(&xml_fields) {
                        if a.contains(":") { a.to_string() } else { EMPTY_STRING }
                    } else {
                        EMPTY_STRING
                    };
                    addr
                } else {
                    // Common mapping
                    if xml_key.starts_with("%") && xml_key.matches("%").count() == 1 {
                        // Mapping that correlates to a parameter
                        let param = "param".to_string() + xml_key.replace("%", "").as_str();
                        evt.get(&param).unwrap_or(&EMPTY_STRING).to_string()
                    } else {
                        // Normal mapping
                        evt.get(xml_key).unwrap_or(&EMPTY_STRING).to_string()
                    }
                };
                // Add the device custom labels if device custom values are present
                let label = if k.contains(" Custom ") {
                    format!("{}Label={}", cef_key, xml_key).replace("\"", "")
                } else { EMPTY_STRING };
                // build the k=v cef string
                format!("{:?}={:?}~{}", cef_key.trim(), cef_val.trim(), label.trim()).trim().to_string()
            }
        })
        .map(|x|{
            x
                .split("~")
                .filter(|x|!x.is_empty())
                .map(|x|x.to_string())
                .collect::<Vec<_>>()
                .iter()
                .map(|x|{
                    let components = x
                        .split("=")
                        .filter(|x|!x.is_empty())
                        .map(|x|x.to_string())
                        .collect::<Vec<String>>();
                    (
                        components[0].replace("\"", ""),
                        components[1].replace("\"", "").replace("\\\\", "\\")
                    )
                })
                .collect::<CefObject>()
        })
        .flatten()
        .collect::<CefObject>();

    Some(result)
}

/// Lookup into the Values of the given XML Keys and return one of the Values
fn do_one_of(xml_key: &String, event_data: &EventData) -> String {
    let xml_fields = xml_key
        .split(",")
        .map(|x| x.replace("one_of(", "").replace(")", ""))
        .map(|x| {
            // Get the XML Value(s) from the XML Key(s)
            if x == "localhost" {
                "localhost".to_string()
            } else if x == "No" {
                "No".to_string()
            } else if x == "Blocked" {
                "Blocked".to_string()
            } else {
                event_data.get(&x).unwrap_or(&EMPTY_STRING).to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(",");
    one_of!(xml_fields)
}

/// Lookup into the Values of the given XML Keys and return all of the Values
fn do_all_of(xml_key: &String, event_data: &EventData) -> String {
    let xml_fields = xml_key
        .split(",")
        .map(|x| x.replace("one_of(", "").replace(")", ""))
        .map(|x| {
            // Get the XML Value(s) from the XML Key(s)
            event_data.get(&x).unwrap_or(&EMPTY_STRING).to_string()
        })
        .collect::<Vec<String>>()
        .join(",");
    all_of!(xml_fields)
}

/// Concat all Data params into K:V String
fn do_all_of_data(event_data: &EventData) -> String {
    event_data
        .iter()
        .filter(|(_k, v)| !v.is_empty())
        .map(|(k, v)|{
            format!("{}: {}", k, v)
        })
        .collect::<Vec<String>>()
        .join("\n")
        .replace("\"", "")
}

/// Lookup into the Values of the given XML Keys and return a concat message
fn do_msg_concat(xml_key: &String, event_data: &EventData) -> String {
    let msg = xml_key
        .split(",")
        .map(|x|{
            let x = x
                .replace("concatenate(", "")
                .replace(")", "");
            if x.starts_with("%") {
                let param = "param".to_string() + x.replace("%", "").as_str();
                event_data.get(&param).unwrap_or(&EMPTY_STRING).trim().to_string()
            } else {
                x
            }
        })
        .collect::<Vec<_>>()
        .join("")
        .replace("\"", "");
    msg
}
