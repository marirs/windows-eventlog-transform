use serde::{Deserialize, Deserializer};

pub(crate) fn keywords_map<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(|x| {
        match x.as_str() {
            "0x8020000000000000" => "Audit Success",
            "0x8010000000000000" => "Audit Failure",
            "0x80000000000000" => "Classic",
            _ => &x
        }
            .into()
    })
}
