use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

use crate::{
    de::EventInfo,
    mappings::get_map
};

fn get_event_mapping(event_id: usize) -> String {
    let events = include_str!("../../assets/events.csv").trim();
    let events_map: HashMap<usize, String> = get_map(events, '\n', ',', (0, 1));

    if let Some(event_name) = events_map.get(&event_id) {
       event_name.to_string()
    } else {
        format!("EventID-{}", event_id)
    }
}

pub(crate) fn eventid_map<'de, D>(deserializer: D) -> Result<EventInfo, D::Error>
    where
        D: Deserializer<'de>,
{
    usize::deserialize(deserializer).map(|x| {
        // i used a match, you can use a HashMap with lazy_static
        EventInfo {
            EventID: x,
            EventName: get_event_mapping(x),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_events_map() {
        let evt = get_event_mapping(4781);
        assert_eq!(evt, "The name of an account was changed");
        let evt = get_event_mapping(1);
        assert_eq!(evt, "EventID-1")
    }
}
