use regex::Regex;
use operation_mode::OperationMode;

pub fn is_match(op_mode: &OperationMode, text: &str, pos: usize) -> bool {
    let text = &text[pos..];
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
        assert!(is_match(&OperationMode::new_raw("w"), "w",0));
        assert!(!is_match(&OperationMode::new_raw("e"), "w",0));
        assert!(is_match(&OperationMode::new_raw("w"), "awa",0));
        assert!(is_match(
            &OperationMode::new_raw("potato"),
            "i has potato yep.",0
        ));
    }

    #[test]
    fn regex_match() {
        assert!(is_match(&OperationMode::new_regex("(a|b)").unwrap(), "a",0));
        assert!(is_match(&OperationMode::new_regex("ab").unwrap(), "ab",0));
        assert!(is_match(&OperationMode::new_regex("(a|b)").unwrap(), "cba",0));
    }
}
