use chrono::{Datelike, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DailyProgressConfig {
    pub questions_answered: i32,
    pub last_time_asked_timestamp: i64,
}

fn load_config() -> Result<DailyProgressConfig, confy::ConfyError> {
    let config: DailyProgressConfig = confy::load("rlrn", Some("daily_progress_config"))?;

    Ok(config)
}

fn set_config(config: DailyProgressConfig) -> Result<(), confy::ConfyError> {
    confy::store("rlrn", Some("daily_progress_config"), config).unwrap();

    Ok(())
}

pub fn reset_config() -> Result<(), confy::ConfyError> {
    let path = confy::get_configuration_file_path("rlrn", Some("daily_progress_config"))?;

    let path = path.to_str().expect("Failed to convert path to string");

    std::fs::remove_file(path)
        .or_else(|error| -> Result<(), std::io::Error> {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    // file not found, so it's already reset
                    return Ok(());
                }
                _ => {
                    return Err(error);
                }
            }
        })
        .expect("Failed to remove file");

    Ok(())
}

pub fn do_i_need_to_answer_question_now() -> bool {
    let mut daily_progress_config = load_config().expect("Failed to load daily progress config");

    let now_timestamp = Local::now().timestamp();
    let now = NaiveDateTime::from_timestamp(now_timestamp, 0);
    let time_of_last_answer =
        NaiveDateTime::from_timestamp(daily_progress_config.last_time_asked_timestamp, 0);

    let shall_reset_counter = now.day() != time_of_last_answer.day()
        || now.month() != time_of_last_answer.month()
        || now.year() != time_of_last_answer.year();

    if shall_reset_counter {
        daily_progress_config.questions_answered = 0;
        daily_progress_config.last_time_asked_timestamp = now_timestamp;
        set_config(daily_progress_config).expect("Failed to update daily progress config");
        return true;
    }

    let user_config =
        crate::configs::user_config::load_config().expect("Failed to load user config");

    if daily_progress_config.questions_answered < user_config.daily_goal {
        let seconds_left_from_last_answer =
            now_timestamp - daily_progress_config.last_time_asked_timestamp;
        let hours_left_from_last_answers = seconds_left_from_last_answer as f64 / 60.0 / 60.0;

        if hours_left_from_last_answers > user_config.reminder_interval as f64 {
            daily_progress_config.last_time_asked_timestamp = now_timestamp;
            set_config(daily_progress_config).expect("Failed to update daily progress config");
            return true;
        }
    }

    false
}

pub fn update_daily_progress(answered_correctly: bool) {
    let mut daily_progress_config = load_config().expect("Failed to load daily progress config");

    let now_timestamp = Local::now().timestamp();
    daily_progress_config.last_time_asked_timestamp = now_timestamp;

    if answered_correctly {
        daily_progress_config.questions_answered += 1;
    }

    set_config(daily_progress_config).expect("Failed to update daily progress");
}
