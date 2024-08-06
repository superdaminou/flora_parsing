use log::debug;

#[derive(Debug, PartialEq, Eq)]
pub struct Data{pub time: u32, pub action: String}

impl Data {
    pub fn csv_compliant(&self) -> String {    
        self.time.to_string() + ";"+ &self.action
    }
    
    pub fn prettier(&self) -> String {
        "Time: ".to_string() + &self.time.to_string() + ", action: " + &self.action.to_string()
    }
}

impl From<(u32, &str)> for Data {
    fn from(value: (u32, &str)) -> Self {
        Data {time: value.0, action: value.1.to_string()}
    }
}

impl From<(&str, &str)> for Data {
    fn from(value: (&str, &str)) -> Self {
        debug!("Data from: {}", value.0.to_string()+ ":" + value.1);
        Data {time: value.0.parse::<u32>().unwrap(), action: value.1.to_string()}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_from() {
        let data = Data::from(("32", "autre"));
        assert_eq!(data.time, 32);
        assert_eq!(data.action, "autre".to_string())
    }

    #[test]
    fn data_to_csv() {
        let result = Data::from(("32", "autre")).csv_compliant();
        assert_eq!(result, "32;autre".to_string());
    }

    #[test]
    fn data_to_prettier() {
        let result = Data::from(("32", "autre")).prettier();
        assert_eq!(result, "Time: 32, action: autre".to_string());
    }

}