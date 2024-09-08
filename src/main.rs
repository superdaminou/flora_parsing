// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use complete_data::{CompleteData, ToContent};
use data::Data;
use error::ParsingError;
use rows::Rows;
mod rows;
use dotenv::dotenv;
use log::info;
mod complete_data;
mod data;
mod error;

fn main() {
    dotenv().ok();
    env_logger::init();

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![execute])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

type Group= Vec<String>;
type Groups= Vec<Group>;

 
 fn extract_groups(content: &str) -> Result<Groups, ParsingError>{
    info!("Starting group extraction");
    let mut groups : Groups = vec![];
    let lines = content.lines();

    let mut group : Group = vec![];
    lines.filter(|l|!l.trim().is_empty()).for_each(|l| {
        if !l.starts_with(" ") {
            groups.push(group.clone());
            group = vec![];
            group.push(l.to_string());
        } else {
            group.push(l.to_string())
        }
    });
    groups.push(group.clone());
    Ok(groups)
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


#[tauri::command]
fn execute(content: &str, mode: &str ) -> Result<String, ParsingError> {
    let groups : Groups = extract_groups(content)?;
    

    let complete_datas = groups.iter()
        .map(CompleteData::try_from)
        .collect::<Result<Vec<CompleteData>,ParsingError>>()?;


    info!("Executing  {}", mode);
    let output_content = complete_datas.iter()
        .map(|data|{
            match mode {
                "total_action" => data.total_by_action(),
                "action_time" => data.cumul_action(),
                "csv" => data.prettier(),
                _ => panic!("Unknonw mode {}", mode)
            }
        })
        .collect::<Vec<String>>()
        .join("\r\n");
    Ok(output_content)
}