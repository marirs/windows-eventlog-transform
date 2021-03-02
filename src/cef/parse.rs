use crate::{
    de::{Event, EventData},
    cef::properties,
};
use std::collections::HashMap;


pub(crate) fn get_class_id(evt: Event) -> String {
    //! Gets the Class Id from the Event.
    //! ClassId will be the `Provider.Name` if present,
    //! else it will be `System.Channel`
    let provider = if let Some(p) = evt.System.Provider {
        p
    } else { return evt.System.Channel };

    let name = if let Some(n) = provider.Name {
        n
    } else { evt.System.Channel };

    name
}

pub(crate) fn get_event_outcome(keyword: String) -> String {
    //! Get the Event Outcome
    match keyword.as_str() {
        "Audit Success" => "/Success",
        "Audit Failure" => "/Failure",
        _ => ""
    }
        .into()
}

pub(crate) fn get_event_data(event_id: usize, event_data: EventData) -> String {
    let mut result: String = String::new();
    let binary_data = if let Some(b) = event_data.Binary {
        b
    } else {
        String::new()
    };
    let event_data = event_data.Data;

    if event_data.is_array() {
        // Event data is a array, could be empty or string values
        let arr = event_data
            .as_array()
            .unwrap()
            .iter()
            .map(|a|a.to_string().replace("\"", ""))
            .collect::<Vec<String>>();
        let arr = format!("cs6Label=EventData cs6={}", arr.join(",")).clone();
        result = arr;
    } else if event_data.is_object() {
        // Event data has k:v pair
        let obj = event_data.as_object().unwrap();
        let obj = obj
            .iter()
            .map(|(k, v)|(k.as_str(), v.as_str().unwrap_or("")))
            .map(|(k, v)|{
                (k.trim_matches('\\'), v.trim_matches('\\'))
            })
            .map(|(k, v)|(k.to_string(), v.to_string()))
            .collect::<HashMap<String, String>>();

        // convert to cef string
        let parsed_cef = properties::mapper(&event_id, &obj);
        println!("{:#?}", parsed_cef);
        // let parsed_cef = properties::event_mappings(event_id, obj.clone());
        // TODO: remaining fields to key=val conversion
        // let cef = obj
        //     .iter()
        //     .map(|(k, v)| format!("{}={}", k, v))
        //     .collect::<Vec<String>>()
        //     .join(" ");
        // result = format!("{} cs6Label=EventData cs6=", parsed_cef);
    }

    if !binary_data.is_empty() && !result.is_empty() {
        // both binary & event data is available
        format!("{} cs6Label=BinaryData cs6={}", result, binary_data)
    } else if binary_data.is_empty() && !result.is_empty() {
        // binary data is empty, but event data is present
        format!("{}", result)
    } else if !binary_data.is_empty() && result.is_empty() {
        // binary data is represent, but event data is empty
        format!("cs6Label=BinaryData cs6={}", binary_data)
    } else {
        // both binary data & event data are empty
        String::new()
    }
}