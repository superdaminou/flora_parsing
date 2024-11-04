use strum_macros::{Display, EnumString};

use super::complete_data::{CompleteData, ToContent};

#[derive(EnumString, Display)]
pub enum Mode {
    #[strum(serialize = "total_action")]
    TotalAction,
    #[strum(serialize = "action_time")]
    ActionTime,
    #[strum(serialize = "csv")]
    CSV
}

impl Mode {
    pub fn execute(&self, data: &CompleteData) -> String {
        match self {
            Mode::TotalAction => data.total_by_action(),
            Mode::ActionTime => data.cumul_action(),
            Mode::CSV => data.csv_prettier()
        }
    }
}
