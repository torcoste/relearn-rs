pub fn reset_command_handler() {
    crate::configs::user_config::reset_config().expect("Failed to reset user config");
    crate::configs::daily_progress_config::reset_config()
        .expect("Failed to reset daily progress config");

    println!("Your progress and settings have been successfully reset!");
}
