use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub daily_goal: i32,
    pub reminder_interval: i32,
}

impl ::std::default::Default for UserConfig {
    fn default() -> Self {
        Self {
            daily_goal: 5,
            reminder_interval: 1,
        }
    }
}

pub fn load_config() -> Result<UserConfig, confy::ConfyError> {
    let config: UserConfig = confy::load("rlrn", Some("user_config"))?;

    Ok(config)
}

pub fn set_config(config: UserConfig) -> Result<(), confy::ConfyError> {
    confy::store("rlrn", Some("user_config"), config).unwrap();

    Ok(())
}

pub fn reset_config() -> Result<(), confy::ConfyError> {
    let path = confy::get_configuration_file_path("rlrn", Some("user_config"))?;

    let path = path.to_str().expect("Failed to convert path to string");

    std::fs::remove_file(path).expect("Failed to remove file");

    Ok(())
}
