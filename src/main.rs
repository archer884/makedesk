mod entry;

use std::{
    fs::File,
    io::{self, Write},
    process,
};

use clap::Parser;
use entry::{Entry, EntryFormatter, EntryType};

#[derive(Debug, Parser)]
struct Args {
    /// entry name
    name: String,

    /// entry exec command
    exec: String,

    /// entry type
    #[arg(short = 't', long = "type")]
    entry_type: Option<EntryType>,

    /// entry icon path
    #[arg(short, long)]
    icon: Option<String>,

    /// entry comment
    #[arg(short, long)]
    comment: Option<String>,

    /// open terminal for entry
    #[arg(short = 'T', long)]
    terminal: bool,

    /// entry location
    #[arg(short, long)]
    location: Option<String>,
}

impl Args {
    fn entry(&self) -> EntryFormatter<Self> {
        EntryFormatter::new(self)
    }
}

impl Entry for Args {
    fn entry_type(&self) -> EntryType {
        self.entry_type.unwrap_or_default()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn exec(&self) -> &str {
        &self.exec
    }

    fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    fn terminal(&self) -> bool {
        self.terminal
    }
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> io::Result<()> {
    if let Some(location) = args.location.as_deref() {
        let mut f = File::create_new(location)?;
        writeln!(f, "{}", args.entry())?;
    } else {
        println!("{}", args.entry());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Args;

    #[test]
    fn can_build_duke3d() {
        let args = Args {
            name: "Duke Nukem".into(),
            exec: "/home/username/eduke/eduke32 -j /home/username/eduke".into(),
            entry_type: None,
            icon: Some("/home/username/duke-round.png".into()),
            comment: Some("I'm all out of bubble gum.".into()),
            terminal: false,
            location: None,
        };

        let actual = args.entry().to_string();
        let expected = include_str!("../resource/Duke Nukem.desktop");
        assert_eq!(actual, expected);
    }
}
