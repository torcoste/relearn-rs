use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView};
use rand::Rng; // 0.8.5

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Question {
    question: String,
    answers: Vec<String>,
    correct_answer: String,
}

pub fn learn_command_handler(_force: bool) {
    let mut select = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center)
        // Use keyboard to jump to the pressed letters
        .autojump();

    // Let's override the `j` and `k` keys for navigation
    let mut siv = cursive::default();
    // Read the list of cities from separate file, and fill the view with it.
    // (We include the file at compile-time to avoid runtime read errors.)
    let json_content = include_str!("assets/questions.json");
    let questions: Vec<Question> = serde_json::from_str(json_content).unwrap();

    let num = rand::thread_rng().gen_range(0..questions.len());
    let question = questions[num].clone();
    select.add_all_str(question.answers.clone());
    // Sets the callback for when "Enter" is pressed.
    select.set_on_submit(move |s, item: &str| {
        if item == question.correct_answer {
            s.pop_layer();
            s.add_layer(Dialog::info("Correct!"));
        } else {
            s.add_layer(Dialog::info("Wrong!"));
        }
    });

    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });
    // Let's add a ResizedView to keep the list at a reasonable size
    // (it can scroll anyway).
    siv.add_layer(Dialog::around(select.scrollable()).title(question.question));

    siv.run();
}
