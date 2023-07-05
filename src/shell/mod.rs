use core::fmt;

pub mod files;

#[derive(Debug)]
pub struct ShellError(String);

pub type ShellResult = Result<String, ShellError>;

pub fn execute(command: &str) -> String {
    let mut cmd = command.split(' ');
    let executable = match cmd.next() {
        Some(e) => e,
        None => "",
    };
    let args: Vec<&str> = cmd.collect();

    let result: ShellResult = match executable {
        "echo" => echo(args),
        "help" => help(args),
        "ls" => files::ls(args),
        "cat" => files::cat(args),
        _ => Ok("no command found".to_owned()),
    };

    match result {
        Ok(r) => r,
        Err(ShellError(err)) => err,
    }
}

fn echo(args: Vec<&str>) -> ShellResult {
    if args.len() == 0 {
        return Ok("<span class=red>No arguments given.</span>".to_owned());
    }
    let msg = args.join(" ");
    Ok(msg)
}

fn help(args: Vec<&str>) -> ShellResult {
    let mut helptext = r#"<span class=purple>shoudev site command prompt</span>
Avaliable commands:"#
        .to_owned();

    helptext = format!("{}\n{}", helptext, help_entry("help", "shows help message"));
    helptext = format!("{}\n{}", helptext, help_entry("echo", "repeats text"));

    Ok(helptext)
}

fn help_entry(command: &str, desc: &str) -> String {
    format!("<span class=orange>{}</span> - {}", command, desc)
}

impl From<fmt::Error> for ShellError {
    fn from(_: fmt::Error) -> Self {
        ShellError("error".to_owned())
    }
}
