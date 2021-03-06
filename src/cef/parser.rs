use chrono::DateTime;

use std::collections::HashMap;

use crate::{
    de::{Event, EventData},
    mappers::{
        cef_map::CefMap,
        load_mapping,
        EventMappingGetters,
    },
    cef::utils::*,
};

type CefObject = HashMap<String, String>;

const EMPTY_STRING: String = String::new();

pub(crate) fn build_cef_extension(evt: &Event) -> CefObject {
    //! Build the CEF Extension Object
    let start = DateTime::parse_from_rfc3339(evt.System.TimeCreated.as_str());
    let start = if let Ok(e) = start {
        e.timestamp().to_string()
    } else {
        evt.System.TimeCreated.to_string()
    };
    let external_id= evt.System.Event.EventID;
    let outcome = get_event_outcome(&evt.System.Keywords);

    let event_data = get_event_data(&external_id, evt.EventData.as_ref());

    let result = [
        ("start".to_string(), start),
        ("external_id".to_string(), external_id.to_string()),
        ("outcome".to_string(), outcome),
        ("cn5Label".to_string(), "EventRecordId".to_string()),
        ("cn5".to_string(), evt.System.EventRecordID.to_string()),
        ("cs2Label".to_string(), "Keywords".to_string()),
        ("cs2".to_string(), evt.System.Keywords.to_string()),
    ]
        .iter()
        .map(|(k, v)|(k.to_string(), v.to_string()))
        .collect::<HashMap<String, String>>()
        .into_iter()
        .chain(event_data)
        .filter(|(k,_v)|!k.is_empty())
        .collect::<HashMap<String, String>>();

    result
}

pub(crate) fn get_class_id(evt: &Event) -> String {
    //! Gets the Class Id from the Event.
    //! ClassId will be the `Provider.Name` if present,
    //! else it will be `System.Channel`
    let provider = if let Some(p) = &evt.System.Provider {
        p
    } else { return evt.clone().System.Channel };

    let name = if let Some(n) = provider.clone().Name {
        n
    } else { evt.clone().System.Channel };

    name
}

pub(crate) fn get_event_outcome(keyword: &String) -> String {
    //! Get the Event Outcome
    match keyword.as_str() {
        "Audit Success" => "/Success",
        "Audit Failure" => "/Failure",
        _ => ""
    }
        .into()
}

/// Parse the Event data and return a CEF Object of it
fn get_event_data(event_id: &usize, event_data: Option<&EventData>) -> CefObject {
    let mut result: HashMap<String, String> = HashMap::new();

    if let Some(data) = event_data {
        let evt_data = &data.Data;
        if evt_data.is_array() {
            // Event Data is an Array
            let arr = evt_data
                .as_array()
                .unwrap()
                .iter()
                .map(|a|a.to_string().replace("\"", ""))
                .collect::<Vec<String>>();
            result.insert("cs6Label".into(), "EventData".into());
            result.insert("cs6".into(), arr.join(","));
        } else if evt_data.is_object() {
            let obj = evt_data.as_object().unwrap();
            let obj = obj
                .iter()
                .map(|(k, v)|(k.as_str(), v.as_str().unwrap_or("")))
                .map(|(k, v)|{
                    (k.trim_matches('\\'), v.trim_matches('\\'))
                })
                .map(|(k, v)|(k.to_string(), v.to_string()))
                .collect::<HashMap<String, String>>();

            // convert to cef string
            let parsed_cef = mapper(&event_id, &obj);
            let parsed_cef = if let Some(cef) = parsed_cef {
                cef
            } else {
                HashMap::new()
            };
            result = parsed_cef
        }

        // Return the formated CEF HashMap
        result
    } else {
        // No Event Data found
        result
    }
}

/// Map & convert the Events into a CEF HashMap
fn mapper(event_id: &usize, event_data: &HashMap<String, String>) -> Option<CefObject> {
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
                    format!("msg={}", do_msg_concat(xml_key, &event_data))
                } else if xml_key.eq_ignore_ascii_case("all_of_data") {
                    // All_Of_Data found for message
                    format!("msg={}", do_all_of_data(&event_data))
                } else {
                    // Normal message
                    format!("msg={}", xml_key)
                }
            } else if k.eq_ignore_ascii_case("device action"){
                // Got a Device Action Message
                if xml_key.matches(" ").count() > 0 {
                    format!("act={}", xml_key)
                } else {
                    format!("act={}", event_data.get(xml_key).unwrap_or(&EMPTY_STRING).to_string())
                }
            } else {
                // Mapping if not a message field
                let cef_key = cef_map.get_cef_field_or_default(&k);
                let cef_val = if xml_key.starts_with("one_of(") {
                    // One_Of(..) value encountered
                    do_one_of(xml_key, &event_data)
                } else if xml_key.starts_with("all_of(") {
                    // One_Of(..) value encountered
                    do_all_of(xml_key, &event_data)
                } else if xml_key.starts_with("both(") {
                    // Both(..) value encountered
                    xml_key
                        .split(",")
                        .map(|x| x.replace("both (", "").replace(")", ""))
                        .map(|x| {
                            event_data.get(&x).unwrap_or(&EMPTY_STRING).to_string()
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
                    let addr = if let Some(a) = event_data.get(&xml_fields) {
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
                        event_data.get(&param).unwrap_or(&EMPTY_STRING).to_string()
                    } else {
                        // Normal mapping
                        event_data.get(xml_key).unwrap_or(&EMPTY_STRING).to_string()
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
