use serde::{Deserialize, Deserializer};

use crate::{
    de::EventInfo,
    mappers::*,
};

fn get_event_name_mapping(event_id: usize) -> String {
    let map = load_mapping();

    if let Some(name) = map.get_name(&event_id) {
       name.to_string()
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
            EventName: get_event_name_mapping(x),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_events_map() {
        let evt = get_event_name_mapping(4781);
        assert_eq!(evt, "The name of an account was changed:");
        let evt = get_event_name_mapping(1);
        assert_eq!(evt, "EventID-1")
    }
}
