use std::fmt::Write;

use crate::statics::FILES;

use super::ShellResult;

pub struct SimFile {
    pub name: String,
    pub contents: String,
    pub permissions: FilePerms,
}

pub struct FilePerms {
    pub r: bool,
    pub w: bool,
    pub x: bool,
}

impl Default for SimFile {
    fn default() -> Self {
        Self {
            name: String::default(),
            contents: String::default(),
            permissions: FilePerms::default(),
        }
    }
}

impl Default for FilePerms {
    fn default() -> Self {
        Self {
            r: true,
            w: true,
            x: false,
        }
    }
}

pub fn ls(args: Vec<&str>) -> ShellResult {
    let mut output: String = String::new();

    if args.contains(&"-l") {
        for file in FILES.iter() {
            let read = if file.permissions.r { "r" } else { "-" };
            let write = if file.permissions.w { "w" } else { "-" };
            let execute = if file.permissions.x { "x" } else { "-" };
            write!(output, "{}{}{} {}\n", read, write, execute, file.name)?;
        }
    } else {
        for file in FILES.iter() {
            write!(output, "{} ", file.name)?;
        }
    }
    Ok(output)
}

pub fn cat(args: Vec<&str>) -> ShellResult {
    if args.len() != 1 {
        return Ok("This command only accepts one argument.".to_owned());
    }

    let filename = args[0];

    let file = FILES.iter().find(|f| f.name == filename);

    match file {
        Some(f) => Ok(f.contents.to_owned()),
        None => Ok(format!("File \"{}\" not found.", filename)),
    }
}
