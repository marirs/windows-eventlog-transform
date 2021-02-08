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
    }
}