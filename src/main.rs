use std::{env, fs::{read_to_string, File}, io::Write, vec};

use complete_data::{CompleteData, ToContent};
use data::Data;
use rows::Rows;
mod rows;
use dotenv::dotenv;
use log::info;
mod complete_data;
mod data;

fn main() {
    dotenv().ok();
    env_logger::init();

    let mut args =  env::args().skip(1);

    let mode = args.next().expect("Expecting a mode to run");
    let input = args.next().expect("Expecting a file to read");
    let output = args.next().unwrap_or("./output.txt".to_string());

    let groups : Groups = extract_groups(&input);
    

    let complete_datas = groups.iter()
        .map(CompleteData::from)
        .collect::<Vec<CompleteData>>();


    info!("Executing  {}", mode);
    let output_content = complete_datas.iter()
        .map(|data|{
            match mode.as_str() {
                "total_action" => data.total_by_action(),
                "action_time" => data.cumul_action(),
                "csv" => data.prettier(),
                _ => panic!("Unknonw mode {}", mode)
            }
        })
        .collect::<Vec<String>>()
        .join("\r\n");

    File::create(output)
            .and_then(|mut f|f.write_all(output_content.as_bytes())).unwrap()
}

type Group= Vec<String>;
type Groups= Vec<Group>;

 
 fn extract_groups(input: &str) -> Groups{
    info!("Reading from {}", input);
    let mut groups : Groups = vec![];
    let binding = read_to_string(input).unwrap();
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
    groups
 }

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


