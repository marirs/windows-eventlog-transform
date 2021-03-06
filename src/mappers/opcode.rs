use serde::{Deserialize, Deserializer};

pub(crate) fn opcode_map<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    usize::deserialize(deserializer).map(|x| {
        let ustr = x.to_string();
        match x {
            0 => "Info",
            1 => "Start",
            2 => "Stop",
            3 => "Data Collection Start",
            4 => "Data Collection Stop",
            5 => "Extension",
            6 => "Reply",
            7 => "Resume",
            8 => "Suspend",
            9 => "Send",
            240 => "Receive",
            _ => &ustr
        }
            .into()
    })
}
