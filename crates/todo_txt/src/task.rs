use crate::parse;
use std::str::FromStr;

//------------------------------------------------------------------------------
//
// Task
//
//------------------------------------------------------------------------------

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Task {
    pub completed: bool,
    pub priority: Option<char>,
    pub date_completed: Option<chrono::NaiveDate>,
    pub date_created: Option<chrono::NaiveDate>,

    description: String,
    contexts: Vec<String>,
    projects: Vec<String>,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.completed {
            write!(f, "x ")?;
        }
        if let Some(p) = self.priority {
            write!(f, "({}) ", p)?;
        }
        if let Some(d) = self.date_completed {
            write!(f, "{} ", d)?;
        }
        if let Some(d) = self.date_created {
            write!(f, "{} ", d)?;
        }
        write!(f, "{}", self.description)
    }
}

impl FromStr for Task {
    type Err = parse::ParseTaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let task_parser = parse::TaskParser::new();
        if let Some(task) = task_parser.parse_line(s) {
            Ok(task)
        } else {
            Err(parse::ParseTaskError)
        }
    }
}

impl Task {
    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
        let tokens: std::collections::VecDeque<&str> =
            self.description.split_whitespace().collect();
        for token in tokens {
            if token.starts_with('@') {
                self.contexts
                    .push(token.strip_prefix('@').unwrap().to_string());
            } else if token.starts_with('+') {
                self.projects
                    .push(token.strip_prefix('+').unwrap().to_string());
            }
        }
    }

    pub fn contexts(&self) -> &[String] {
        &self.contexts
    }

    pub fn projects(&self) -> &[String] {
        &self.projects
    }
}
