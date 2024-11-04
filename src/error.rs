use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("An error occured: {0}")]
    DefaultError(String),
}

impl serde::Serialize for ParsingError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
