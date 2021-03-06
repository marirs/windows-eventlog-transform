#![allow(dead_code)]
use std::collections::HashMap;

const EMPTY_STRING: String = String::new();

/// Get the first int from a string
fn parse_int(input: &str) -> Option<usize> {
    input
        .chars()
        .skip_while(|ch| !ch.is_digit(10))
        .take_while(|ch| ch.is_digit(10))
        .fold(None, |acc, ch| {
            ch.to_digit(10).map(|b| acc.unwrap_or(0) * 10 + b as usize)
        })
}

/// Get the next usable/missing number
fn next_usable_number(arr: Vec<usize>) -> usize {
    let mut arr = arr;
    arr.sort();
    for (i, num) in arr.iter().enumerate() {
        if i+1 != *num{
            return i+1;
        }
    }
    1
}

/// Get the next CNn
pub(crate) fn get_next_cn(cn: &Vec<String>) -> String {
    let available_cn_num = cn
        .iter()
        .map(|x|parse_int(x).unwrap_or(0))
        .collect::<Vec<_>>();
    let next_cn_num = next_usable_number(available_cn_num);
    format!("cn{}", next_cn_num)
}

/// Get the next CSn
pub(crate) fn get_next_cs(cs: &Vec<String>) -> String {
    let available_cs_num = cs
        .iter()
        .map(|x|parse_int(x).unwrap_or(0))
        .collect::<Vec<_>>();
    let next_cs_num = next_usable_number(available_cs_num);
    format!("cs{}", next_cs_num)
}


/// Lookup into the Values of the given XML Keys and return one of the Values
pub(crate) fn do_one_of(xml_key: &String, event_data: &HashMap<String, String>) -> String {
    let xml_fields = xml_key
        .split(",")
        .map(|x| x.replace("one_of(", "").replace(")", ""))
        .map(|x| {
            // Get the XML Value(s) from the XML Key(s)
            if x == "localhost" {
                "localhost".to_string()
            } else if x == "No" {
                "No".to_string()
            } else if x == "Blocked" {
                "Blocked".to_string()
            } else {
                event_data.get(&x).unwrap_or(&EMPTY_STRING).to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(",");
    one_of!(xml_fields)
}

/// Lookup into the Values of the given XML Keys and return all of the Values
pub(crate) fn do_all_of(xml_key: &String, event_data: &HashMap<String, String>) -> String {
    let xml_fields = xml_key
        .split(",")
        .map(|x| x.replace("one_of(", "").replace(")", ""))
        .map(|x| {
            // Get the XML Value(s) from the XML Key(s)
            event_data.get(&x).unwrap_or(&EMPTY_STRING).to_string()
        })
        .collect::<Vec<String>>()
        .join(",");
    all_of!(xml_fields)
}

/// Concat all Data params into K:V String
pub(crate) fn do_all_of_data(event_data: &HashMap<String, String>) -> String {
    event_data
        .iter()
        .filter(|(_k, v)| !v.is_empty())
        .map(|(k, v)|{
            format!("{}: {}", k, v)
        })
        .collect::<Vec<String>>()
        .join("\n")
        .replace("\"", "")
}

/// Lookup into the Values of the given XML Keys and return a concat message
pub(crate) fn do_msg_concat(xml_key: &String, event_data: &HashMap<String, String>) -> String {
    let msg = xml_key
        .split(",")
        .map(|x|{
            let x = x
                .replace("concatenate(", "")
                .replace(")", "");
            if x.starts_with("%") {
                let param = "param".to_string() + x.replace("%", "").as_str();
                event_data.get(&param).unwrap_or(&EMPTY_STRING).trim().to_string()
            } else {
                x
            }
        })
        .collect::<Vec<_>>()
        .join("")
        .replace("\"", "");
    msg
}

/// Get all CNn Keys from the given object
pub(crate) fn get_cn_keys(obj: &HashMap<String, String>) -> Vec<String> {
    get_keys(&obj, "cn")
}

/// Get all CSn Keys from the given object
pub(crate) fn get_cs_keys(obj: &HashMap<String, String>) -> Vec<String> {
    get_keys(&obj, "cs")
}

/// Get Keys with a given filter
fn get_keys(obj: &HashMap<String, String>, filter_by: &str) -> Vec<String> {
    obj
        .keys()
        .filter(|k|k.starts_with(filter_by))
        .filter(|k|!k.ends_with("Label"))
        .map(|k|k.to_string())
        .collect::<Vec<_>>()
}