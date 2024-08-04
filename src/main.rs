use std::{fs::read_to_string, u32, vec};

use complete_data::CompleteData;
use rows::Rows;
mod rows;
use dotenv::dotenv;
use log::{info, debug};
mod complete_data;
fn main() {
    dotenv().ok();
    env_logger::init();

    let mut groups : Groups = vec![];
    let binding = read_to_string("./file.txt").unwrap();
    let lines = binding.lines();


    let mut group : Group = vec![];
    lines.filter(|l|!l.is_empty()).for_each(|l| {
        if !l.starts_with(" ") {
            groups.push(group.clone());
            group = vec![];
            group.push(l.to_string());
        } else {
            group.push(l.to_string())
        }
    });
    groups.push(group.clone());

    let complete_datas = groups.iter()
        .map(|l| CompleteData::from(l))
        .collect::<Vec<CompleteData>>();
    complete_datas.iter().for_each(|datas| {
        println!("{}",datas.pretty_render());
    } )
}

type Group= Vec<String>;
type Groups= Vec<Group>;



type Header = String;
#[derive(Debug)]
enum Value {
    SimpleValue(String),
    Rows(Rows)
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::SimpleValue(value) => value.to_string(),
            Value::Rows(rows) => rows.to_string(),
        }
    }
}


#[derive(Debug)]
pub struct Data{time: u32,action: String}
impl ToString for Data {
    fn to_string(&self) -> String {
        "time: ".to_string() + &self.time.to_string() + " action: " + &self.action.to_string() + "\r\n"
    }
}

impl From<(u32, &str)> for Data {
    fn from(value: (u32, &str)) -> Self {
        Data {time: value.0, action: value.1.to_string()}
    }
}

impl From<(&str, &str)> for Data {
    fn from(value: (&str, &str)) -> Self {
        debug!("Data from: {}", value.0.to_string()+ ":" + &value.1);
        Data {time: value.0.parse::<u32>().unwrap(), action: value.1.to_string()}
    }
}