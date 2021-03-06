use serde::{Deserialize, Deserializer};

pub(crate) fn level_map<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    usize::deserialize(deserializer).map(|x| {
        from_usize_to_string(&x)
    })
}

pub(crate) fn from_usize_to_string(level: &usize) -> String {
    let ustr = level.to_string();
    match level {
        0 | 4 => "Information",
        1 => "Critical",
        2 => "Error",
        3 => "Warning",
        5 => "Verbose",
        _ => &ustr
    }
        .into()
}

pub(crate) fn from_string_to_usize(level: &String) -> usize {
    match level.as_str() {
        "Information" => 2,
        "Critical" => 10,
        "Error" => 7,
        "Warning" => 6,
        "Verbose" => 4,
        _ => 0
    }
}