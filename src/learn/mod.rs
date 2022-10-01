


use cursive::align::HAlign;


use cursive::views::{Dialog, TextView};
use rand::Rng; // 0.8.5

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Question {
    question: String,
    answers: Vec<String>,
    correct_answer: String,
}

pub fn learn_command_handler(_force: bool) {
    // Let's override the `j` and `k` keys for navigation
    let _siv = cursive::default();
    // Read the list of cities from separate file, and fill the view with it.
    // (We include the file at compile-time to avoid runtime read errors.)
    let json_content = include_str!("assets/questions.json");
    let questions: Vec<Question> = serde_json::from_str(json_content).unwrap();

    let num = rand::thread_rng().gen_range(0..questions.len());
    let question = questions[num].clone();
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    let table_view = TextView::new(&question.question).h_align(HAlign::Left);
    let mut dialog = Dialog::around(table_view).title("RLRN");

    // add buttons
    for answer in &question.answers {
        let answer = answer.clone();
        let question = question.clone();

        dialog = dialog.button(answer.clone(), move |s| {
            if answer.clone() == question.correct_answer {
                s.add_layer(
                    Dialog::info("Correct!")
                        .dismiss_button("Next")
                        .button("Quit", |s| s.quit()),
                );
            } else {
                s.add_layer(Dialog::info("Wrong!"));
            }
        });
    }

    siv.add_layer(dialog);

    // Starts the event loop.
    siv.run();
}
