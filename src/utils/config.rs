extern crate toml;

use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::cmp::Ord;
use std::borrow::Borrow;

use iron::typemap::Key;
use self::toml::value::{Table, Value};

#[derive(Clone)]
pub struct Config(Table);

impl Config {

    pub fn new(config_file_name: &str) -> Config {

        let path = Path::new(config_file_name);
        let mut file = File::open(&path).unwrap();
        let mut temp = String::new();

        file.read_to_string(&mut temp).unwrap();

        let table = toml::from_str(&temp).unwrap();

        Config(table)
    }

    pub fn get<T: ?Sized>(&self, key: &T) -> &Value where String: Borrow<T>, T: Ord {
        
        self.0.get(key).unwrap()
    }

    pub fn value(&self) -> Table {

        self.0.clone()
    }
}

impl Key for Config {

    type Value = Config;
}