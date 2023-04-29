use lazy_static::lazy_static;

use crate::shell::files::SimFile;

pub static SHELL_STATUS: &'static str =
    "╭─<span class=green>you@shoudev.com</span> in <span class=blue>~</span>";
pub static SHELL_PROMPT: &'static str = "╰─$ ";

pub static LOGO: &'static str = r#"       __                          __                    
      /\ \                        /\ \                   
  ____\ \ \___     ___   __  __   \_\ \     __   __  __  
 /',__\\ \  _ `\  / __`\/\ \/\ \  /'_` \  /'__`\/\ \/\ \ 
/\__, `\\ \ \ \ \/\ \L\ \ \ \_\ \/\ \L\ \/\  __/\ \ \_/ |
\/\____/ \ \_\ \_\ \____/\ \____/\ \___,_\ \____\\ \___/ 
 \/___/   \/_/\/_/\/___/  \/___/  \/__,_ /\/____/ \/__/

Type <span class=orange>help</span> to see all avaliable commands."#;

lazy_static! {
    pub static ref FILES: Vec<SimFile> = vec![
        SimFile {
            name: "test".to_owned(),
            contents: "Hi this is a test!\n".to_owned(),
            ..Default::default()
        },
        SimFile {
            name: "test2".to_owned(),
            contents: "Hi this is a second test\n".to_owned(),
            ..Default::default()
        }
    ];
}
