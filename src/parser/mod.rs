use regex::Regex;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParseResult<'source, T>
where
    T: Clone + Debug + PartialEq + PartialOrd,
{
    pub value: T,
    pub source: Source<'source>,
}
impl<'source, T> ParseResult<'source, T>
where
    T: Clone + Debug + PartialEq + PartialOrd,
{
    pub fn new(value: T, source: Source<'source>) -> Self {
        Self { value, source }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Source<'source> {
    pub string: &'source str,
    pub index: usize, // in the book this typed as number (f64), maybe this might bite us back l8r
}
impl<'source> Source<'source> {
    pub fn new(string: &'source str) -> Self {
        Self::with_index(string, 0)
    }

    pub fn with_index(string: &'source str, index: usize) -> Self {
        Self { string, index }
    }

    pub fn sticky_match(&self, regexp: &Regex) -> Option<ParseResult<'source, &'source str>> {
        // note that we are slicing the string slice again here to make the `^` in regex to work as
        // expected. i wasted a lot of time investigating the difference in the behavior between
        // TypeScript (i.e., ECMAScript) RegExp and Rust Regex crate.
        // For details, see the `test_source_sticky_match()` below.
        regexp.find(&self.string[self.index..]).map(|m| {
            ParseResult::new(
                m.as_str(),
                Self::with_index(self.string, self.index + m.len()),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_sticky_match() {
        // see how this test code differs in behavior with the following playground snippets:
        // * Rust: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=02b9ab10ea2f51c8612dcc3c86ddf7f3
        // * TypeScript: https://www.typescriptlang.org/play/?#code/DYUwLgBAFghgnhAvBARDd6BGH0GMUBQBokATgIxIQD0MA3gKwC+m1cA3MeBKQExW1GTXG04EKAOmAwAzmACSAOwAmIAB5UADJ1wB7RTN2gpugOYAKWHAkBbGGFxRzFAJQQAhImSKArsGAunJLSckqqGsgAbDr6hsbAZpbwtvaOzuRuXt5+AWJ8UrIKKupaMQZGICYWVikOTnyZXhC+-oHivAWhxREQ0QR65fGJNXZ1zrxuntmt7EA
        // TypeScript's behavior is correct in the context of this crate.

        let source = Source::new("aaaaabaaaaac");
        assert_eq!(source.index, 0); // just to make sure...

        let regex_ab = Regex::new(r"^a{5}b").unwrap();
        let regex_ac = Regex::new(r"^a{5}c").unwrap();

        let result_ab = source.sticky_match(&regex_ab);
        assert!(result_ab.is_some());
        let result_ab = result_ab.unwrap();
        assert_eq!(result_ab.value, "aaaaab");
        assert_eq!(result_ab.source.index, 6);

        let result_ac = source.sticky_match(&regex_ac);
        assert!(result_ac.is_none());

        let result_ac_2 = result_ab.source.sticky_match(&regex_ac);
        assert!(result_ac_2.is_some());
        let result_ac_2 = result_ac_2.unwrap();
        assert_eq!(result_ac_2.value, "aaaaac");
        assert_eq!(result_ac_2.source.index, 12);
    }
}
