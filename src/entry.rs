use core::fmt;
use std::str;

use uncased::UncasedStr;

pub trait Entry {
    fn entry_type(&self) -> EntryType;
    fn name(&self) -> &str;
    fn exec(&self) -> &str;
    fn icon(&self) -> Option<&str>;
    fn comment(&self) -> Option<&str>;
    fn terminal(&self) -> bool;
}

pub struct EntryFormatter<'a, T> {
    entry: &'a T,
}

impl<'a, T> EntryFormatter<'a, T> {
    pub fn new(entry: &'a T) -> Self {
        Self { entry }
    }
}

impl<T: Entry> fmt::Display for EntryFormatter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Desktop Entry]")?;
        writeln!(f, "Type={}", self.entry.entry_type())?;
        writeln!(f, "Name={}", self.entry.name())?;
        writeln!(f, "Exec={}", self.entry.exec())?;

        if let Some(icon) = self.entry.icon() {
            writeln!(f, "Icon={icon}")?;
        }

        if let Some(comment) = self.entry.comment() {
            writeln!(f, "Comment={comment}")?;
        }

        writeln!(f, "Terminal={}", self.entry.terminal())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum EntryType {
    #[default]
    Application,
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntryType::Application => f.write_str("Application"),
        }
    }
}

impl str::FromStr for EntryType {
    type Err = ParseEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match UncasedStr::new(s).as_ref() {
            "application" => Ok(EntryType::Application),
            _ => Err(ParseEntryError(s.into())),
        }
    }
}

#[derive(Debug)]
pub struct ParseEntryError(String);

impl fmt::Display for ParseEntryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown entry type: {}", self.0)
    }
}

impl std::error::Error for ParseEntryError {}
