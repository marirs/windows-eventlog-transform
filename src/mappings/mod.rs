use std::collections::HashMap;

pub(crate) mod levels;
pub(crate) mod keywords;
pub(crate) mod events;
pub(crate) mod opcode;
#[allow(dead_code)]
pub(crate) mod tasks;

/// Common function to convert mapping data into Hashmap
pub(crate) fn get_map(
    text_str: &str,
    line_break: char, string_break: char,
    (key_index, val_index): (usize, usize)
) -> HashMap<usize, String> {
    text_str
        .trim()
        .split(line_break)
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|l| {
            // Convert to a tuple (usize, String).
            let components = l
                .split(string_break)
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            (
                components[key_index].clone().parse::<usize>().unwrap(),
                components[val_index].clone(),
            )
        })
        .collect()
}
