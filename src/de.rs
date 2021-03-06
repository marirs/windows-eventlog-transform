#![allow(non_snake_case)]

use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

use crate::{
    mappers::{
        event_name::eventid_map,
        keywords::keywords_map,
        levels::level_map,
        opcode::opcode_map,
        tasks::tasks_map,
    }
};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub xmlns: String,
    pub System: System,
    pub EventData: Option<EventData>,
    pub UserData: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct System {
    pub Provider: Option<Provider>,
    #[serde(default = "default_device_vendor")]
    pub DeviceVendor: String,
    pub EventRecordID: usize,
    #[serde(alias = "EventID", deserialize_with = "eventid_map")]
    pub Event: EventInfo,
    #[serde(deserialize_with = "level_map")]
    pub Level: String,
    #[serde(deserialize_with = "tasks_map")]
    pub Task: String,
    #[serde(deserialize_with = "opcode_map")]
    pub Opcode: String,
    #[serde(deserialize_with = "keywords_map")]
    pub Keywords: String,
    #[serde(deserialize_with = "flatten_time_created")]
    pub TimeCreated: String,
    pub Correlation: Correlation,
    pub Execution: Execution,
    pub Channel: String,
    pub Computer: String,
    pub Security: Option<Security>,
    pub Version: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    pub Name: Option<String>,
    pub Guid: Option<String>,
    pub EventSourceName: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeCreated {
    pub SystemTime: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Correlation {
    pub ActivityID: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Execution {
    pub ProcessID: usize,
    pub ThreadID: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventInfo {
    pub EventID: usize,
    pub EventName: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Security {
    pub UserID: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Data {
    KV {
        Name: String,
        #[serde(default, rename = "$value")]
        Value: Option<String>,
    },
    V {
        #[serde(default, rename = "$value")]
        Value: Option<String>,
    },
}


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventData {
    #[serde(default, deserialize_with = "eventdata_map")]
    pub Data: Value,
    pub Binary: Option<String>,
}

fn eventdata_map<'de, D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: Deserializer<'de>,
{
    Option::<Vec<Data>>::deserialize(deserializer).map(|o| {
        if let Some(evd) = o {
            let mut m = HashMap::new();
            let mut v = Vec::new();

            for d in evd {
                match d {
                    Data::KV { Name, Value } => {
                        m.insert(Name, Value);
                    }
                    Data::V { Value } => {
                        v.push(Value);
                    }
                }
            }
            // remove null from vec
            let v = v
                .iter()
                .filter(|a|!a.as_ref().is_none())
                .map(|a|a.as_ref().unwrap().to_string())
                .collect::<Vec<String>>();

            match (m.is_empty(), v.is_empty()) {
                (true, true) => Default::default(),
                (true, false) => json!(v),
                (false, true) => json!(m),
                (false, false) => json!([m, v]),
            }
        } else {
            Default::default()
        }
    })
}

fn flatten_time_created<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    TimeCreated::deserialize(deserializer).map(|x| x.SystemTime)
}

fn default_device_vendor() -> String {
    "Microsoft".to_string()
}