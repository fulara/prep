use regex::Regex;

#[derive(Clone, Debug)]
pub enum OperationMode {
    RawText(String),
    Regex(Regex),
}

impl OperationMode {
    pub fn new_raw(s: &str) -> OperationMode {
        OperationMode::RawText(s.into())
    }

    pub fn new_regex(s: &str) -> Option<OperationMode> {
        match Regex::new(s) {
            Ok(r) => Some(OperationMode::Regex(r)),
            Err(_) => None,
        }
    }
}
