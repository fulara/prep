use regex::Regex;
use operation_mode::OperationMode;

pub fn is_match(op_mode: &OperationMode, text: &str) -> bool {
    match op_mode {
        &OperationMode::RawText(ref to_match) => text.find(to_match).is_some(),
        &OperationMode::Regex(ref regex) => regex.is_match(text),
    }
}

#[cfg(test)]
mod matching_test {
    use super::*;

    #[test]
    fn text_match() {
        assert!(is_match(&OperationMode::new_raw("w"), "w"));
        assert!(!is_match(&OperationMode::new_raw("e"), "w"));
        assert!(is_match(&OperationMode::new_raw("w"), "awa"));
        assert!(is_match(
            &OperationMode::new_raw("potato"),
            "i has potato yep.",
        ));
    }

    #[test]
    fn regex_match() {
        assert!(is_match(&OperationMode::new_regex("(a|b)").unwrap(), "a"));
        assert!(is_match(&OperationMode::new_regex("ab").unwrap(), "ab"));
        assert!(is_match(&OperationMode::new_regex("(a|b)").unwrap(), "cba"));
    }
}
