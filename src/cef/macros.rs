/// Get `value` of the first value filled string/str
/// from the strings/strs passed.
///
/// ## Example Usage
/// ```rust
/// use winevents_xml_transform::one_of;
///
/// let a = String::new();
/// let b = String::from("b");
/// let c = String::from("c");
/// let one_of_abc = one_of!(a.clone(), b.clone(), c.clone());
/// assert_eq!(one_of_abc, "b".to_string());
///
/// let one_of_cb = one_of!(c.clone(), b.clone());
/// assert_eq!(one_of_cb, "c".to_string());
/// ```
#[macro_export]
macro_rules! one_of {
    ($x:expr) => {{
        let mut v = $x.split(",").map(|x|x.to_string()).collect::<Vec<String>>();
        let x = v
            .drain(..)
            .skip_while(|x|x.is_empty())
            .next()
            .unwrap_or_default();
        x
    }};
    ($($x:expr),+) => {{
        let mut v = Vec::new();
        $(
            v.push(
                if $x.to_string() == "-".to_string() {"".to_string()} else {$x.to_string()}
            );
        )*
        let x = v
            .drain(..)
            .skip_while(|x|x.is_empty())
            .next()
            .unwrap_or_default();
        x
    }};
}

/// Get `value` of the first value filled string/str
/// from the strings/strs passed.
///
/// ## Example Usage
/// ```rust
/// use winevents_xml_transform::all_of;
///
/// let a = String::new();
/// let b = String::from("b");
/// let c = String::from("c");
/// let all_of_abc = all_of!(a.clone(), b.clone(), c.clone());
/// assert_eq!(all_of_abc, "b c".to_string());
///
/// let all_of_cb = all_of!(c.clone(), b.clone());
/// assert_eq!(all_of_cb, "c b".to_string());
/// ```
#[macro_export]
macro_rules! all_of {
    ($x:expr) => {{
        let v = $x.split(",").map(|x|x.to_string()).collect::<Vec<String>>();
        let v = v.join(" ").trim().to_string();
        v
    }};
    ($($x:expr),+) => {{
        let mut v = Vec::new();
        $(
            v.push(
                if $x.to_string() == "-".to_string() {"".to_string()} else {$x.to_string()}
            );
        )*
        let v = v.join(" ").trim().to_string();
        v
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_of() {
        let a = "";
        let b = "b";
        let c = "c";
        let one_of_abc = one_of!(a.clone(), b.clone(), c.clone());
        assert_eq!(one_of_abc, "b".to_string());

        let one_of_cb = one_of!(c.clone(), b.clone());
        assert_eq!(one_of_cb, "c".to_string());

        let one_of_ab_strings = one_of!(a.to_string(), b.to_string());
        assert_eq!(one_of_ab_strings, "b".to_string());

        let a = ",b,c";
        let one_of_abc = one_of!(a);
        assert_eq!(one_of_abc, "b".to_string())
    }

    #[test]
    fn test_all_of() {
        let a = String::new();
        let b = String::from("b");
        let c = String::from("c");
        let all_of_abc = all_of!(a.clone(), b.clone(), c.clone());
        assert_eq!(all_of_abc, "b c".to_string());

        let all_of_cb = all_of!(c.clone(), b.clone());
        assert_eq!(all_of_cb, "c b".to_string());

        let all_ac = all_of!(a.clone(), c.clone());
        assert_eq!(all_ac, "c".to_string());
    }
}