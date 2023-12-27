
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;

use crate::task::Task;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTaskError;

lazy_static! {
    static ref RE_PRIORITY: Regex = Regex::new(r"^\(([A-Z])\)$").unwrap();
}

//------------------------------------------------------------------------------
//
// Parsing state machine
//
//------------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Token<'a> {
    Complete,
    Priority(char),
    Date(chrono::NaiveDate),
    Word(&'a str),
}

impl<'a> Token<'a> {
    fn lex(s: &'a str) -> Token<'a> {
        if s == "x" {
            Token::Complete
        } else if RE_PRIORITY.is_match(s) {
            Token::Priority(s.chars().nth(1).unwrap())
        } else if let Ok(naive_date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Token::Date(naive_date)
        } else {
            Token::Word(s)
        }
    }
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Complete => write!(f, "x"),
            Token::Priority(p) => write!(f, "({})", p),
            Token::Date(d) => write!(f, "{}", d),
            Token::Word(w) => write!(f, "{}", w),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseState {
    Start,
    PastCompletion,
    PastPriority,
    PastDate1,
    PastDate2,
    InDescription,
}

pub struct TaskParser {
    state: ParseState,
    task: Task,
}

impl TaskParser {
    pub fn new() -> Self {
        TaskParser {
            state: ParseState::Start,
            task: Default::default(),
        }
    }

    pub fn parse_line(mut self, line: &str) -> Option<Task> {

        let mut token_iter = line.split_whitespace();

        while let Some(token) = token_iter.next() { 
            self.state = self.next(Token::lex(token));
            if self.state == ParseState::InDescription {
                self.task.set_description(
                    format!("{} {}", 
                        token, 
                        //token_iter.remainder().unwrap_or("") // Not in stable yet :(
                        token_iter.collect::<Vec<_>>().join(" ") 
                    ).trim()
                );
                break;
            }
        }

        if self.state == ParseState::Start {
            None
        } else {
            Some(self.task)
        }
    }

    fn next(&mut self, token: Token) -> ParseState {
        match (&self.state, token) {
            // Start state explicit transitions
            (ParseState::Start, Token::Complete) => {
                self.task.completed = true;
                ParseState::PastCompletion
            }
            (ParseState::Start, Token::Priority(p)) => {
                self.task.priority = Some(p);
                ParseState::PastPriority
            }
            (ParseState::Start, Token::Date(date)) => {
                self.task.date_created = Some(date);
                ParseState::PastDate1
            }

            // PastCompletion explicit transitions
            (ParseState::PastCompletion, Token::Priority(p)) => {
                self.task.priority = Some(p);
                ParseState::PastPriority
            }
            (ParseState::PastCompletion, Token::Date(date)) => {
                self.task.date_created = Some(date);
                ParseState::PastDate1
            }

            // PastPriority explicit transitions
            (ParseState::PastPriority, Token::Date(date)) => {
                self.task.date_created = Some(date);
                ParseState::PastDate1
            }

            // PastDate1 explicit transitions
            (ParseState::PastDate1, Token::Date(date)) => {
                self.task.date_completed = self.task.date_created;
                self.task.date_created = Some(date);
                ParseState::PastDate2
            }
            (_, _) => {
                ParseState::InDescription
            }

            /*
            // Default case -- move to description and append
            (_, _) => {
                let s = token.to_string();
                if s.starts_with('@') {
                    self.task
                        .contexts
                        .push(s.strip_prefix('@').unwrap().to_string());
                } else if s.starts_with('+') {
                    self.task
                        .projects
                        .push(s.strip_prefix('+').unwrap().to_string());
                }
                //self.task.description.push_str(&s);
                self.task.description = format!("{} {}", self.task.description, &s);
                ParseState::InDescription
            }
            */
        }
    }
}

impl Default for TaskParser {
    fn default() -> Self {
        Self::new()
    }
}
