use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::{self, Read}};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub balance: u64,
    pub id: String
}

pub enum PullDataError {
    FileNotFound,
    FileNotParsed,
}

pub fn pull_persons_data () -> Result<Vec<Person>, io::Error> {
    let mut fd = OpenOptions::new().read(true).write(true).open("Persons.json")?;
    let mut buffer = String::new();
    fd.read_to_string(&mut buffer).unwrap();
    let data: Vec<Person> = serde_json::from_str(&buffer)?;
    Ok(data)
}