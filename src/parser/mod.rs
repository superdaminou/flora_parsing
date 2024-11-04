use complete_data::CompleteData;
use log::info;
use rows::Rows;
use crate::error::ParsingError;

pub mod complete_data;
pub mod data;
pub mod rows;
mod mode;
pub use mode::Mode;


pub fn parse(content: &str, mode: Mode) -> Result<String, ParsingError> {
    let groups: Groups = extract_groups(content)?;
    
    let complete_datas = groups
        .iter()
        .map(CompleteData::try_from)
        .collect::<Result<Vec<CompleteData>, ParsingError>>()?;
    
    let output_content = complete_datas
        .iter()
        .map(|data| mode.execute(data))
        .collect::<Vec<String>>()
        .join("\r\n");
    
    Ok(output_content)
}

type Group = Vec<String>;
type Groups = Vec<Group>;

fn extract_groups(content: &str) -> Result<Groups, ParsingError> {
    info!("Starting group extraction");
    let mut groups: Groups = vec![];
    let lines = content.lines();

    let mut group: Group = vec![];
    lines.filter(|l| !l.trim().is_empty()).for_each(|l| {
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
    Rows(Rows),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::SimpleValue(value) => value.to_string(),
            Value::Rows(rows) => rows.to_string(),
        }
    }
}
