use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use csv::ReaderBuilder;

#[allow(dead_code)]
pub(crate) mod cef_map;
pub(crate) mod levels;
pub(crate) mod keywords;
pub(crate) mod event_name;
pub(crate) mod opcode;
pub(crate) mod tasks;

type EventMapping = HashMap<usize, WinEvent>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WinEvent {
    /// Event Id
    event_id: usize,
    /// Category of the Event
    category: String,
    /// Sub Category of the Event
    sub_category: String,
    /// Possible outcome of the Event
    outcome: String,
    /// Channel
    channel: String,
    /// Name of the Event
    name: String,
    /// Mapping information of the Event
    mapping_info: Option<HashMap<String, String>>
}

pub fn load_mapping() -> EventMapping {
    //! Loads a default Windows EventLog Native Mappings Config
    let event_mapping = include_str!("../../assets/event_mappings.csv");
    ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_reader(event_mapping.as_bytes())
        .records()
        .filter(|result|result.is_ok())
        .map(|result| {
            let mut fields =
                result
                    .iter()
                    .flatten()
                    .filter_map(|r| if r.is_empty() { None } else { Some(r) });
            let event_id = fields.next().and_then(|x| x.parse::<usize>().ok()).unwrap();
            let category = fields.next().unwrap().to_string();
            let sub_category = fields.next().unwrap().to_string();
            let outcome = fields.next().unwrap().to_string();
            let channel = fields.next().unwrap().to_string();
            let name = fields.next().unwrap().to_string();
            let mapping_info = fields
                .map(|x| {
                    let components = x.splitn(2, ':')
                        .map(|x| x.replace("'", "").trim().to_string())
                        .collect::<Vec<String>>();
                    (components[0].to_string(), components[1].to_string())
                })
                .collect::<HashMap<String, String>>();
            let e = WinEvent {
                event_id,
                category,
                sub_category,
                outcome,
                channel,
                name,
                mapping_info: if mapping_info.is_empty() { None } else { Some(mapping_info) },
            };
            (event_id, e)
        })
        .collect()
}

pub trait EventMappingGetters {
    fn get_info(&self, event_id: &usize) -> Option<&WinEvent>;
    fn get_mapping_info(&self, event_id: &usize) -> Option<&HashMap<String, String>>;
    fn get_name(&self, event_id: &usize) -> Option<&String>;
    fn get_channel(&self, event_id: &usize) -> Option<&String>;
    fn get_category(&self, event_id: &usize) -> Option<&String>;
    fn get_sub_category(&self, event_id: &usize) -> Option<&String>;
    fn get_outcome(&self, event_id: &usize) -> Option<&String>;
}

impl EventMappingGetters for EventMapping {
    fn get_info(&self, event_id: &usize) -> Option<&WinEvent> {
        //! Get the Event Information
        self.get(event_id).map(|x| x)
    }

    fn get_mapping_info(&self, event_id: &usize) -> Option<&HashMap<String, String>> {
        //! Get the Mapping Information of the Event
        let x = self
            .get(event_id)
            .map(|x|x.mapping_info.as_ref());
        if let Some(x) = x {
            x
        } else {
            None
        }
    }

    fn get_name(&self, event_id: &usize) -> Option<&String> {
        //! Get the Name of the Event
        self.get(event_id).map(|x| &x.name)
    }

    fn get_channel(&self, event_id: &usize) -> Option<&String> {
        //! Get the Channel/Provider Source of the Event
        self.get(event_id).map(|x| &x.channel)
    }

    fn get_category(&self, event_id: &usize) -> Option<&String> {
        //! Get the Category of the Event
        self.get(event_id).map(|x| &x.category)
    }

    fn get_sub_category(&self, event_id: &usize) -> Option<&String> {
        //! Get the Sub Category of the Event
        self.get(event_id).map(|x| &x.sub_category)
    }

    fn get_outcome(&self, event_id: &usize) -> Option<&String> {
        //! Get the outcome (success/failure) of the Event
        self.get(event_id).map(|x| &x.outcome)
    }
}

#[cfg(test)]
mod tests {
    use crate::mappers::{load_mapping, EventMappingGetters};

    #[test]
    fn test_mappings() {
        let event_map = load_mapping();
        let event_name = event_map.get_info(&4760);
        assert!(event_name.is_some());
        let event_name = event_name.unwrap();
        assert_eq!(event_name.name, "A security-disabled universal group was changed.".to_string());
        assert!(event_name.mapping_info.is_some());
        assert_eq!(event_name.channel, "Security")
    }
}
