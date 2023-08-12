use regex::Regex;

use tracing::debug;

use crate::command_line_args::CommandLineArgs;

use std::borrow::Cow;

pub struct RegexProcessor {
    regex: Option<regex::Regex>,
}

impl RegexProcessor {
    pub fn new(command_line_args: &CommandLineArgs) -> Self {
        let regex = match &command_line_args.regex {
            None => None,
            Some(regex) => Some(Regex::new(regex).unwrap()),
        };
        Self { regex }
    }

    pub fn process_string<'a>(&self, arg: &'a str, input_line: &'a str) -> Cow<'a, str> {
        debug!(
            "in process_string arg = {:?} input_line = {:?}",
            arg, input_line
        );

        let regex = match &self.regex {
            None => return Cow::from(arg),
            Some(regex) => regex,
        };

        let captures = match regex.captures(&input_line) {
            None => return Cow::from(arg),
            Some(captures) => captures,
        };

        debug!("captures = ${:?}", captures);

        let mut dest = String::new();

        captures.expand(&arg, &mut dest);

        debug!(
            "after expand input_line = {:?} dest = {:?}",
            input_line, dest
        );

        Cow::from(dest)
    }
}
