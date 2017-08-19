use regex::Regex;
use operation_mode::OperationMode;

pub struct Replacer {
    mode: OperationMode,
    to: String,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ReplaceResult<'a> {
    pub before: &'a str,
    pub after: &'a str,
    pub old: &'a str,
    pub new: String,
    pub position_of_replacement: usize,
}

impl Replacer {
    pub fn new(mode: OperationMode, to: &str) -> Replacer {
        Replacer {
            mode: mode,
            to: to.into(),
        }
    }

    pub fn replace<'a>(&self, text: &'a str, start: usize) -> ReplaceResult<'a> {
        match self.mode {
            OperationMode::RawText(ref from) => {
                if let Some(pos) = text[start..].find(from) {
                    let position_of_replacement = pos + start;
                    ReplaceResult {
                        before: &text[0..position_of_replacement],
                        after: &text[position_of_replacement + from.len()..],
                        new: self.to.clone(),
                        old: &text[position_of_replacement..position_of_replacement + from.len()],
                        position_of_replacement: position_of_replacement + self.to.len(),
                    }
                } else {
                    panic!("replacer should only receive cases where replacing happens.");
                }
            }
            OperationMode::Regex(ref regex) => {
                if let Some(mat) = regex.find_at(text, start) {
                    let new = regex.replace(&text[mat.start()..mat.end()], &*self.to).into_owned();

                    let position = mat.start() + new.len();
                    ReplaceResult {
                        before: &text[0..mat.start()],
                        after: &text[mat.end()..],
                        new: new,
                        old: &text[mat.start()..mat.end()],
                        position_of_replacement: position,
                    }
                } else {
                    panic!("replacer should only receive cases where replacing happens.")
                }
            }
        }
    }
}

#[cfg(test)]
mod replace_test {
    use super::*;

    fn assert<'a>(expected: &str, expected_pos: usize, result: ReplaceResult<'a>) {
        let resulting_string = format!("{}{}{}", result.before, result.new, result.after);
        assert_eq!(expected, resulting_string);
        assert_eq!(expected_pos, result.position_of_replacement);
    }

    #[test]
    fn text_replace() {
        let r = Replacer::new(OperationMode::new_raw("w"), "e");
        assert("eewe", 1, r.replace("wewe", 0));
        assert("eeee", 3, r.replace("eewe", 1));
    }

    #[test]
    fn regex_replace() {
        let r = Replacer::new(OperationMode::new_regex("(a|b)").unwrap(), "c");
        assert("c", 1, r.replace("a", 0));
        assert("cdbdad", 1, r.replace("adbdad", 0));
        assert("cdcdad", 3, r.replace("cdbdad", 0));
        assert("cdcdcd", 5, r.replace("cdcdad", 0));
    }

    #[test]
    fn regex_backreference() {
        let r = Replacer::new(OperationMode::new_regex("(a|b)").unwrap(), "$1$1");
        let result = r.replace("adb", 0);
        let pos = result.position_of_replacement;
        assert("aadb", 2, result);
        assert("aadbb", 5, r.replace(&"aadb", pos));
    }
}
