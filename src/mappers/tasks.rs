use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

fn get_map(
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

fn get_sa_tasks_mapping(task_id: usize) -> String {
    let tasks = include_str!("../../assets/tasks.csv").trim();
    let tasks_map: HashMap<usize, String> = get_map(
        tasks, '\n', ',', (1,2)
    );

    if let Some(task_name) = tasks_map.get(&task_id) {
        task_name.to_string()
    } else {
        format!("({})", task_id)
    }
}

pub(crate) fn tasks_map<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    usize::deserialize(deserializer).map(|x| {
        let mapped = get_sa_tasks_mapping(x);
        match x {
            0 => "None",
            _ => &mapped
        }
            .into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sa_tasks_map() {
        let evt = get_sa_tasks_mapping(12289);
        assert_eq!(evt, "Security System Extension");
        let evt = get_sa_tasks_mapping(1);
        assert_eq!(evt, "(1)")
    }
}
