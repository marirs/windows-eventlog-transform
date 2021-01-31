#![allow(non_snake_case)]
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;

use std::collections::HashMap;

use crate::{
    mappings::{
        levels::level_map,
        keywords::keywords_map,
        events::eventid_map,
        opcode::opcode_map,
        tasks::tasks_map,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    pub Name: Option<String>,
    pub Guid: Option<String>,
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
pub struct System {
    pub Provider: Option<Provider>,
    #[serde(alias = "EventID", deserialize_with = "eventid_map")]
    pub Event: EventInfo,
    pub Version: usize,
    #[serde(deserialize_with = "level_map")]
    pub Level: String,
    #[serde(deserialize_with = "tasks_map")]
    pub Task: String,
    #[serde(deserialize_with = "opcode_map")]
    pub Opcode: String,
    #[serde(deserialize_with = "keywords_map")]
    pub Keywords: String,
    pub TimeCreated: TimeCreated,
    pub EventRecordID: usize,
    pub Correlation: Correlation,
    pub Execution: Execution,
    pub Channel: String,
    pub Computer: String,
    pub Security: Option<Security>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    pub CbsPackageChangeState: Option<CbsPackageChangeState>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CbsPackageChangeState {
    pub PackageIdentifier: String,
    pub IntendedPackageState: usize,
    pub IntendedPackageStateTextized: String,
    pub ErrorCode: String,
    pub Client: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    Name: String,
    #[serde(rename = "$value")]
    Value: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventData {
    pub Data: Option<Vec<Data>>,
    pub Binary: Option<String>,
}

fn EventData_to_HashMap<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<String, Option<String>>>, D::Error>
    where
        D: Deserializer<'de>,
{
    EventData::deserialize(deserializer).map(|x| {
        x.Data
            .map(|mut x| x.drain(..).map(|x| (x.Name, x.Value)).collect())
    })
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub xmlns: String,
    pub System: System,
    #[serde(default, deserialize_with = "EventData_to_HashMap")]
    pub EventData: Option<HashMap<String, Option<String>>>,
    pub UserData: Option<UserData>,
}
