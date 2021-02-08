use std::collections::HashMap;

#[allow(dead_code)]
pub(crate) struct CefMap {
    pub(crate)  map: HashMap<String, Vec<String>>,
    // Reciprocal of `map`
    pub(crate)  inverted_map: HashMap<String, String>,
}

impl CefMap {
    fn get_cef_value_hash(mut cef_val: String) -> String {
        // Remove all whitespaces
        cef_val.retain(|c| !c.is_whitespace());
        // Turn all to lowercase
        cef_val.to_lowercase()
    }

    fn populate_map(text: String) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let lines: Vec<String> = text
            .trim()
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect();

        for line in lines.into_iter() {
            let components: Vec<String> = line.split(',').map(|x| x.to_string()).collect();
            map.insert(components[0].clone(), (&components[1..]).to_vec());
        }
        map
    }

    fn populate_inverted_map(map: &HashMap<String, Vec<String>>) -> HashMap<String, String> {
        let mut inv_map: HashMap<String, String> = HashMap::new();

        for (k, v) in map.iter() {
            let inverted_hash = Self::get_cef_value_hash(v[0].to_string());
            inv_map.insert(inverted_hash, k.clone());
        }

        inv_map
    }

    pub(crate)  fn from_mapping(text: &str) -> Self {
        //! Load the given CEF Mapping CSV String
        //!
        //! ## Example usage
        //! ```ignore
        //! let cef_map = CefMap::from_mapping(include_str!("../../assets/cef_mapping.csv"));
        //! ```
        let text = text.trim();
        let map = Self::populate_map(text.to_string());
        let inverted_map = Self::populate_inverted_map(&map);
        Self { map, inverted_map }
    }

    pub(crate)  fn get_cef_field_for(&self, field: &str) -> Option<String> {
        //! Get the CEF field name by passing any corresponding field value
        //!
        //! ## Example usage
        //! ```ignore
        //! let cef_map = CefMap::from_mapping(include_str!("../../assets/cef_mapping.csv"));
        //! let cef_field_name = cef_map.get_cef_field_for("deviceAction");
        //! ```
        let inverted_hash = Self::get_cef_value_hash(field.to_string());
        self.inverted_map.get(&inverted_hash).map(|x| x.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cef_field_for() {
        let cef_map = CefMap::from_mapping(include_str!("../../assets/cef_mapping.csv"));
        let cef_field_name = cef_map.get_cef_field_for("deviceAction");
        let expected = Some(String::from("act"));
        assert_eq!(cef_field_name, expected);

        let cef_field_name = cef_map.get_cef_field_for("Destination User ID");
        let expected = Some(String::from("duid"));
        assert_eq!(cef_field_name, expected);

        let cef_field_name = cef_map.get_cef_field_for("");
        assert_eq!(cef_field_name, None);
    }
}
