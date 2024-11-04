use std::collections::HashMap;

use crate::error::ParsingError;

use super::data::Data;

#[derive(Debug)]
pub struct Rows(Vec<Row>);

impl ToString for Rows {
    fn to_string(&self) -> String {
        self.0.iter().map(Row::to_string).collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Row {
    identifier: String,
    datas: Vec<Data>,
}

impl TryFrom<&String> for Row {
    type Error = ParsingError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let identifier_data = value.trim().split_once(":").unwrap_or(("N/A", "N/A"));
        let datas = identifier_data
            .1
            .split(" ")
            .map(|d| d.trim())
            .filter(|l| !l.is_empty())
            .map(|data| data.split_once(".").unwrap_or_default())
            .map(Data::try_from)
            .collect::<Result<Vec<Data>, ParsingError>>()?;
        Ok(Row {
            identifier: identifier_data.0.to_string(),
            datas,
        })
    }
}

impl ToString for Row {
    fn to_string(&self) -> String {
        self.identifier.to_string()
            + ": \r\n"
            + &self
                .datas
                .iter()
                .map(Data::prettier)
                .reduce(|acc, val| acc.to_owned() + &val)
                .unwrap_or("".to_string())
            + "\r\n"
    }
}

impl TryFrom<Vec<&String>> for Rows {
    type Error = ParsingError;
    fn try_from(values: Vec<&String>) -> Result<Self, ParsingError> {
        let rows = values
            .iter()
            .map(|l| Row::try_from(*l))
            .collect::<Result<_, _>>()?;
        Ok(Rows(rows))
    }
}

impl Rows {
    pub fn total_by_action(&self) -> String {
        let mut sum_by_action: HashMap<String, u32> = HashMap::default();
        self.0
            .iter()
            .map(|row| row.sum_by_action())
            .for_each(|result| {
                result.clone().iter().for_each(|(k, v)| {
                    let sum = sum_by_action.get(k).unwrap_or(&0);
                    sum_by_action.insert(k.to_string(), sum + v);
                });
            });

        "Action;Total Time;\r\n".to_string() + 
        &sum_by_action
            .iter()
            .map(|(action, sum)| {
                action.clone() + ";" + &sum.to_string()+";"
            })
            .collect::<Vec<String>>()
            .join("\r\n")
    }

    pub fn csv_prettier(&self) -> String {
        "Action;Time;\r\n".to_string()
            + &self
                .0
                .iter()
                .map(|r| {
                    r.datas
                        .iter()
                        .map(|d| d.action.clone() + ";" + &d.time.to_string() + ";")
                        .collect::<Vec<String>>()
                        .join("\r\n")
                })
                .collect::<Vec<String>>()
                .join("\r\n")
    }

    

    pub fn cumul_action(&self) -> String {
        let mut cumul: HashMap<Action, Occurence> = HashMap::new();
        let mut cumul_by_time: Vec<(Time, HashMap<Action, Occurence>)> = vec![];

        self.0.iter().for_each(|row| {
            row.datas.iter().for_each(|data| {
                cumul.insert(
                    data.action.clone(),
                    cumul.get(&data.action).unwrap_or(&0) + 1,
                );
                cumul_by_time.push((data.time, cumul.clone()));
            })
        });

        "Time;Action;Occurences;\r\n".to_string() + 
        &cumul_by_time
            .iter()
            .map(|cumul| {
                cumul.0.to_string() + ";;\r\n"
                    + &cumul.1
                        .iter()
                        .map(|f| "  ;".to_string() + f.0 + ";" + &f.1.to_string()+";")
                        .collect::<Vec<String>>()
                        .join("\r\n")
            })
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

type Time=u32;
type Occurence = u32;
type Action = String;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_from() {
        let one_row = &"305: 2147.100 491.100".to_string();
        let two_row = &"310: 118.100 42.300".to_string();
        let rows = Rows::try_from(vec![one_row, two_row]).unwrap();
        assert_eq!(
            rows.0.first().unwrap(),
            &Row {
                identifier: "305".to_string(),
                datas: vec![Data::from((2147, "100")), Data::from((491, "100"))]
            }
        );
        assert_eq!(
            rows.0.get(1).unwrap(),
            &Row {
                identifier: "310".to_string(),
                datas: vec![Data::from((118, "100")), Data::from((42, "300"))]
            }
        );
    }

    #[test]
    fn row_to_string() {
        let row = &Row {
            identifier: "305".to_string(),
            datas: vec![Data::from((2147, "100")), Data::from((491, "100"))],
        }
        .to_string();
        assert_eq!(
            row,
            "305: \r\nTime: 2147, action: 100Time: 491, action: 100\r\n"
        )
    }

    #[test]
    fn prettier_rows() {
        let one_row = &"305: 2147.100 491.100".to_string();
        let two_row = &"310: 118.100 42.300".to_string();
        let rows = Rows::try_from(vec![one_row, two_row]).unwrap();
        assert_eq!(
            rows.csv_prettier(),
            "Action;Time;\r\n100;2147;\r\n100;491;\r\n100;118;\r\n300;42;".to_string()
        )
    }
}
