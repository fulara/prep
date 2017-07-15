use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReplaceMode {
    Text,
    Regex,
}

#[derive(Clone, Debug)]
enum ReplaceModeStorage {
    Text(String),
    Regex(Regex),
}

pub struct Replacer {
    mode: ReplaceModeStorage,
    to: String,
}

impl Replacer {
    fn new(mode: ReplaceMode, from: &str, to: &str) -> Option<Replacer> {
        match mode {
            ReplaceMode::Text => Some(Replacer {
                mode: ReplaceModeStorage::Text(from.into()),
                to: to.into(),
            }),
            ReplaceMode::Regex => {
                match Regex::new(from) {
                    Ok(r) => {
                        Some(Replacer {
                            mode: ReplaceModeStorage::Regex(r),
                            to: to.into(),
                        })
                    }
                    Err(_) => None,
                }
            }
        }
    }

    fn replace(&self, text: &str) -> String {
        match self.mode {
            ReplaceModeStorage::Text(ref from) => text.replace(from, &self.to),
            ReplaceModeStorage::Regex(ref regex) => regex.replace_all(text, &*self.to).into_owned(),
        }
    }
}

#[cfg(test)]
mod replace_test {
    use super::*;

    #[test]
    fn text_replace() {
        let r = Replacer::new(ReplaceMode::Text, "w", "e").unwrap();
        assert_eq!("eeee", r.replace("wewe"));
    }

    #[test]
    fn regex_replace() {
        let r = Replacer::new(ReplaceMode::Regex, "(a|b)", "c").unwrap();
        assert_eq!("cd", r.replace("ad"));
        assert_eq!("cccd", r.replace("abcd"));
    }

    #[test]
    fn regex_backreference() {
        let r = Replacer::new(ReplaceMode::Regex, "(a|b)", "$1$1").unwrap();
        assert_eq!("aad", r.replace("ad"));
    }
}
