pub mod add_to_rc;
use crate::configs::user_config::{load_config, set_config};
use cursive::{
    align::HAlign,
    view::{Nameable, Resizable, Scrollable},
    views::{Button, Dialog, LinearLayout, PaddedView, SelectView, TextView},
};
use std::{cell::RefCell, rc::Rc};

fn get_daily_goal_text(value: i32) -> String {
    format!("Daily goal: {} questions", value)
}

fn get_reminder_interval_text(value: i32) -> String {
    format!("Reminder interval: {} hours", value)
}

fn get_question_tag_text(value: &str) -> String {
    format!("Question tag: {}", value)
}

fn get_question_level_text(value: i32) -> String {
    let level = match value {
        1 => "Junior",
        2 => "Middle",
        3 => "Senior",
        _ => "Unknown",
    };
    format!("Question level: {}", level)
}

pub fn init_command_handler() {
    let config = load_config().expect("Failed to load config");

    let output_config_rc = Rc::new(RefCell::new(config));

    let mut siv = cursive::default();

    let mut day_goal_select = SelectView::new();
    day_goal_select.add_item("5 questions", 5);
    day_goal_select.add_item("10 questions", 10);
    day_goal_select.add_item("15 questions", 15);
    day_goal_select.add_item("20 questions", 20);
    {
        let output_config_rc = output_config_rc.clone();
        day_goal_select.set_on_submit(move |s, value| {
            let mut output_config = output_config_rc.borrow_mut();
            output_config.daily_goal = *value;
            s.call_on_name("day_goal_text", |v: &mut TextView| {
                v.set_content(get_daily_goal_text(*value));
            });

            s.focus_name("reminder_interval_select").unwrap();
        });
    }

    let mut reminder_interval_select = SelectView::new();
    reminder_interval_select.add_item("(each terminal session)", 0);
    reminder_interval_select.add_item("1 hour", 1);
    reminder_interval_select.add_item("2 hours", 2);
    reminder_interval_select.add_item("3 hours", 3);
    reminder_interval_select.add_item("4 hours", 4);
    {
        let output_config_rc = output_config_rc.clone();
        reminder_interval_select.set_on_submit(move |s, value| {
            let mut output_config = output_config_rc.borrow_mut();
            output_config.reminder_interval = *value;
            s.call_on_name("reminder_interval_text", |v: &mut TextView| {
                v.set_content(get_reminder_interval_text(*value));
            });

            s.focus_name("question_tag_select").unwrap();
        });
    }
    let reminder_interval_select = reminder_interval_select.with_name("reminder_interval_select");

    let mut question_tag_select = SelectView::new()
        .item("(all)", ".".to_string())
        .item("Backend", "backend".to_string())
        .item("Frontend", "frontend".to_string())
        .item("Data science", "data-science".to_string());
    {
        let output_config_rc = output_config_rc.clone();
        question_tag_select.set_on_submit(move |s, value: &String| {
            let mut output_config = output_config_rc.borrow_mut();
            output_config.question_tag = value.to_string();

            s.call_on_name("question_tag_text", |v: &mut TextView| {
                v.set_content(get_question_tag_text(value));
            });

            s.focus_name("question_level_select").unwrap();
        });
    }

    let mut question_level_select = SelectView::new()
        .item("Junior", 1)
        .item("Middle", 2)
        .item("Senior", 3);

    {
        let output_config_rc = output_config_rc.clone();
        question_level_select.set_on_submit(move |s, value| {
            let mut output_config = output_config_rc.borrow_mut();
            output_config.question_level = *value;

            s.call_on_name("question_level_text", |v: &mut TextView| {
                v.set_content(get_question_level_text(*value));
            });

            s.focus_name("ok_button").unwrap();
        });
    }

    let layout = {
        LinearLayout::vertical()
            // Daily goal
            .child(
                TextView::new(get_daily_goal_text(
                    output_config_rc.clone().borrow().daily_goal,
                ))
                .h_align(HAlign::Center)
                .with_name("day_goal_text"),
            )
            .child(day_goal_select)
            // Reminder interval
            .child(
                TextView::new(get_reminder_interval_text(
                    output_config_rc.clone().borrow().reminder_interval,
                ))
                .h_align(HAlign::Center)
                .with_name("reminder_interval_text"),
            )
            .child(reminder_interval_select)
            // Question tag
            .child(
                TextView::new(get_question_tag_text(
                    output_config_rc.clone().borrow().question_tag.as_str(),
                ))
                .h_align(HAlign::Center)
                .with_name("question_tag_text"),
            )
            .child(question_tag_select.with_name("question_tag_select"))
            // Question level
            .child(
                TextView::new(get_question_level_text(
                    output_config_rc.clone().borrow().question_level,
                ))
                .h_align(HAlign::Center)
                .with_name("question_level_text"),
            )
            .child(question_level_select.with_name("question_level_select"))
            // Submit button
            .child(
                Button::new("Ok", move |s| {
                    let output_config = output_config_rc.borrow().clone();
                    set_config(output_config).expect("Failed to save config");
                    s.quit();
                })
                .with_name("ok_button"),
            )
    };

    let layout = PaddedView::lrtb(2, 2, 0, 0, layout)
        .scrollable()
        .fixed_width(50);

    let layout = Dialog::around(layout).title("RLRN. User configuration");

    siv.add_layer(layout);

    siv.run();
}
