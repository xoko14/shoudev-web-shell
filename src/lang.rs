use std::{collections::HashMap, sync::Mutex};
use lazy_static::lazy_static;
use serde::Deserialize;

pub type LangHashMap = HashMap<String, String>;
#[derive(Deserialize, Debug)]
pub struct LangStrings{
    en: LangHashMap,
    gl: LangHashMap,
    es: LangHashMap,
}

pub struct LangManager{
    strings: LangStrings,
    lang: Lang
}

pub enum Lang{
    En,
    Gl,
    Es
}

impl LangManager{
    pub fn get(&self, key: &str) -> String{
        match match self.lang{
            Lang::En => &self.strings.en,
            Lang::Gl => &self.strings.gl,
            Lang::Es => &self.strings.es,
        }.get(key){
            Some(v) => v.to_owned(),
            None => key.to_owned(),
        }
    }

    pub fn set_lang(&mut self, lang: Lang){
        self.lang = lang;
    }
}

lazy_static!{
    pub static ref L: Mutex<LangManager> = Mutex::new(
        LangManager {
            strings: toml::from_str(include_str!("strings.toml")).unwrap(),
            lang: Lang::En
        });
}

#[macro_export]
macro_rules! t {
    ($x:expr) => {
        crate::lang::L.lock().unwrap().get($x)
    };
}

#[macro_export]
macro_rules! chlang {
    ($x:expr) => {
        crate::lang::L.lock().unwrap().set_lang($x)
    };
}