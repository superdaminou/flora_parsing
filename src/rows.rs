use std::collections::HashMap;

use log::debug;

use crate::Data;

#[derive(Debug)]
pub struct Rows(Vec<Row>);

impl ToString for Rows {
    fn to_string(&self) -> String {
        self.0.iter().map(Row::to_string).collect()
    }
}

#[derive(Debug)]
pub struct Row{
    identifier: String,
    datas: Vec<Data>
}

impl ToString for Row {
    fn to_string(&self) -> String {
        self.identifier.to_string() + ": \r\n" + 
            &self.datas.iter()
                .map(Data::to_string)
                .reduce(|acc, val| acc.to_owned() + &val)
                .unwrap_or("".to_string()) + "\r\n"
    }
}

impl From<Vec<&String>> for Rows {
    fn from(values: Vec<&String>) -> Self {
        debug!("Creating Rows from: {}", values.iter().map(|s|s.to_string()).reduce(|acc, x| acc + ","+ &x ).unwrap_or_default());
        Rows(values.iter().map(|l| {
            let identifier_data = l.trim().split_once(":").unwrap_or(("N/A", "N/A"));
            let datas = identifier_data.1
                .split(" ")
                .map(|d| d.trim())
                .filter(|l| !l.is_empty())
                .map(|data| data.split_once(".").unwrap_or_default())
                .map(Data::from)
                .collect();


            Row {identifier: identifier_data.0.to_string(), datas: datas}
        }).collect::<Vec<Row>>())
    }
}

impl Rows {
    pub fn pretty_render(&self) -> String { 
        let mut sum_by_action: HashMap<String, u32> = HashMap::default();
        self.0.iter().map(|row| {
            row.sum_by_action()
        }).for_each(|result| {
            result.clone().iter().for_each(|(k,v)| {
                let sum = sum_by_action.get(k).unwrap_or(&0);
                sum_by_action.insert(k.to_string(), sum +v);
            });
        } );

        sum_by_action.iter()
            .map(|(action,sum)| "Action: ".to_string() + &action +", Total Time: " + &sum.to_string())
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}


impl Row {
    pub fn sum_by_action(&self) -> HashMap<String, u32> {
        let mut sum_by_action = HashMap::default();
        self.datas.iter().for_each(|data| {
            let sum = sum_by_action.get(&data.action);
            sum_by_action.insert(data.action.clone(), sum.unwrap_or(&0) + data.time);
        });
        sum_by_action
    }
}

type Action = String;