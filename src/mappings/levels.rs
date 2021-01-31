use serde::{Deserialize, Deserializer};

pub(crate) fn level_map<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    usize::deserialize(deserializer).map(|x| {
        let ustr = x.to_string();
        match x {
            0 => "Information",
            1 => "Critical",
            2 => "Error",
            3 => "Warning",
            4 => "Information",
            5 => "Verbose",
            _ => &ustr
        }
            .into()
    })
}
