use log::debug;

use crate::{rows::Rows, Group, Header, Value};


#[derive(Debug)]
pub struct CompleteData(Header, Value);

impl ToContent for CompleteData {
    fn total_by_action(&self) -> String{
        match &self.1 {
            Value::SimpleValue(v) => self.0.clone() +": " + v + "\r\n",
            Value::Rows(rows) =>self.0.clone() +": \r\n" + &rows.total_by_action() + "\r\n"
        }
    } 

    fn prettier(&self) -> String{
        match &self.1 {
            Value::SimpleValue(v) => self.0.clone() +": " + v + "\r\n",
            Value::Rows(rows) =>self.0.clone() +": \r\n" + &rows.prettier() + "\r\n"
        }
    }   

    fn cumul_action(&self) -> String{
        match &self.1 {
            Value::SimpleValue(v) => self.0.clone() +": " + v + "\r\n",
            Value::Rows(rows) =>self.0.clone() +": \r\n" + &rows.cumul_action()  + "\r\n"
        }
    }  
}


pub trait ToContent {
    fn total_by_action(&self) -> String;
    fn prettier(&self) -> String;
    fn cumul_action(&self) -> String;
}


impl From<&Group> for CompleteData {
    fn from(group: &Group) -> Self {
        debug!("Creating Complete Data From: {}", group.join("\r\n"));
        let key_value: (&str, &str);

        if group.len() == 1 {
            key_value =group.first()
                .and_then(|simple_value| simple_value.split_once(':')).unwrap();
            CompleteData(key_value.0.to_string(), Value::SimpleValue(key_value.1.to_string()))
        } else if group.len() > 1 {
            let key =group.first()
                .and_then(|simple_value| simple_value.split_once(':')).unwrap().0;
            let rows: Rows = Rows::from(group.iter().skip(1).collect::<Vec<&String>>());
            return CompleteData(key.to_string(), Value::Rows(rows));
        } else {
            return CompleteData("N/A".to_string(), Value::SimpleValue("N/A".to_string()));
        }
    }
}


impl ToString for CompleteData {
    fn to_string(&self) -> String {
        self.0.clone() + "\r\n" +  &self.1.to_string()
    }
}
