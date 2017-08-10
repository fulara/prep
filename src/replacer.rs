use regex::Regex;
use operation_mode::OperationMode;

pub struct Replacer {
    mode: OperationMode,
    to: String,
}

impl Replacer {
    pub fn new(mode: OperationMode, to: &str) -> Replacer {
        Replacer {
            mode: mode,
            to: to.into(),
        }
    }

    pub fn replace(&self, text: &str) -> String {
        match self.mode {
            OperationMode::RawText(ref from) => text.replace(from, &self.to),
            OperationMode::Regex(ref regex) => regex.replace_all(text, &*self.to).into_owned(),
        }
    }
}

#[cfg(test)]
mod replace_test {
    use super::*;

    #[test]
    fn text_replace() {
        let r = Replacer::new(OperationMode::new_raw("w"), "e");
        assert_eq!("eeee", r.replace("wewe"));
    }

    #[test]
    fn regex_replace() {
        let r = Replacer::new(OperationMode::new_regex("(a|b)").unwrap(), "c");
        assert_eq!("cd", r.replace("ad"));
        assert_eq!("cccd", r.replace("abcd"));
    }

    #[test]
    fn regex_backreference() {
        let r = Replacer::new(OperationMode::new_regex("(a|b)").unwrap(), "$1$1");
        assert_eq!("aadbb", r.replace("adb"));
    }
}
