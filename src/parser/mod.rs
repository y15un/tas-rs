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

#[derive(Clone, Debug)]
pub enum Parser<'source, T>
where
    T: Clone + Debug + PartialEq + PartialOrd,
{
    Constant {
        parse: fn(Source<'source>, &'source T) -> Option<ParseResult<'source, T>>,
        value: &'source T,
    },
    Error {
        parse: fn(Source<'source>, &'static str) -> !,
        message: &'static str,
    },
    Or {
        parse: fn(Source<'source>, &Self, &Self) -> Option<ParseResult<'source, T>>,
        left: &'source Parser<'source, T>,
        right: &'source Parser<'source, T>,
    },
    Regexp {
        parse: fn(Source<'source>, &'source Regex) -> Option<ParseResult<'source, T>>,
        regex: &'source Regex,
    },
}
impl<'source, T> Parser<'source, T>
where
    T: Clone + Debug + PartialEq + PartialOrd,
{
    pub fn constant(value: &'source T) -> Self {
        Self::Constant {
            // FIXME: i don't like the `value` getting cloned. if i am certain everything that is
            //        returned within ParseResult are under `'source` lifetime, then i can maybe
            //        replace `ParseResult<'source, T>` signature to
            //        `ParseResult<'source, &'source T>`.
            parse: |source, value| Some(ParseResult::new(value.clone(), source)),
            value,
        }
    }

    pub fn error(message: &'static str) -> Self {
        Self::Error {
            // TODO: for now, we ignore Source, but maybe later we could use it to display
            //       offending location and such.
            parse: |_: Source<'source>, message| panic!("{}", message),
            message,
        }
    }

    pub fn or(&'source self, right: &'source Self) -> Self {
        Self::Or {
            parse: |source, left, right| {
                let result = left.parse(source);
                if result.is_some() {
                    result
                } else {
                    right.parse(source)
                }
            },
            left: self,
            right,
        }
    }

    pub fn parse(&self, source: Source<'source>) -> Option<ParseResult<'source, T>> {
        match *self {
            Self::Constant { parse, value } => parse(source, value),
            Self::Error { parse, message } => parse(source, message),
            Self::Or { parse, left, right } => parse(source, left, right),
            Self::Regexp { parse, regex } => parse(source, regex),
        }
    }
}
impl<'source> Parser<'source, &'source str> {
    pub fn regexp(regexp: &'source Regex) -> Self {
        Self::Regexp {
            parse: |source, regex| source.sticky_match(regex),
            regex: regexp,
        }
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
    fn test_parser_or() {
        let source = Source::new("q1w2f3p4g5");

        let regex_alpha = Regex::new(r"^[A-Za-z]").unwrap();
        let regex_digit = Regex::new(r"^[0-9]").unwrap();

        let parser_alpha = Parser::regexp(&regex_alpha);
        let parser_digit = Parser::regexp(&regex_digit);
        let parser_or = parser_alpha.or(&parser_digit);

        while let Some(pr) = parser_or.parse(source) {
            println!("matched: {:?}", pr.value);
            println!("new index: {}", pr.source.index);
        }
    }

    #[test]
    fn test_parser_regexp() {
        let mut source = Source::new("Hello hello!");
        let regex_hello = Regex::new(r"^\s*[Hh]ello\s*").unwrap();

        let parser_regexp = Parser::regexp(&regex_hello);
        while let Some(pr) = parser_regexp.parse(source) {
            println!("matched: {:?}", pr.value);
            println!("new index: {}", pr.source.index);

            source = pr.source;
        }
    }

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
