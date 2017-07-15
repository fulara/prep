use regex::Regex;

enum ReplaceMode {
    Text,
    Regex(Regex),
}

pub struct Replacer {
    mode: ReplaceMode,
}

impl Replacer {
    fn new_raw() -> Replacer {
        Replacer { mode: ReplaceMode::Text }
    }

    fn replace(&self, text: &str, from: &str, to: &str) -> String {
        text.replace(from, to)
    }
}

#[cfg(test)]
mod replace_test {
    use super::*;

    #[test]
    fn text_replace() {
        let r = Replacer::new_raw();

        assert_eq!("eeee", r.replace("wewe", "w", "e"));
    }
}
