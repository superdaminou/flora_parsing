use log::debug;

use crate::error::ParsingError;

#[derive(Debug, PartialEq, Eq)]
pub struct Data {
    pub time: u32,
    pub action: String,
}

impl Data {
    pub fn prettier(&self) -> String {
        "Time: ".to_string() + &self.time.to_string() + ", action: " + &self.action.to_string()
    }
}

impl From<(u32, &str)> for Data {
    fn from(value: (u32, &str)) -> Self {
        Data {
            time: value.0,
            action: value.1.to_string(),
        }
    }
}

impl TryFrom<(&str, &str)> for Data {
    type Error = ParsingError;
    fn try_from(value: (&str, &str)) -> Result<Self, ParsingError> {
        debug!("Data from: {}", value.0.to_string() + ":" + value.1);
        let time = value
            .0
            .parse::<u32>()
            .map_err(|e| ParsingError::DefaultError(e.to_string()))?;
        Ok(Data {
            time,
            action: value.1.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_from() {
        let data = Data::try_from(("32", "autre")).unwrap();
        assert_eq!(data.time, 32);
        assert_eq!(data.action, "autre".to_string())
    }

    #[test]
    fn data_to_prettier() {
        let result = Data::try_from(("32", "autre")).unwrap().prettier();
        assert_eq!(result, "Time: 32, action: autre".to_string());
    }
}
