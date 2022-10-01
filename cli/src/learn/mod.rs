use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, TextView};
use rand::Rng; // 0.8.5
use serde::{Deserialize, Serialize};

use crate::configs::daily_progress_config::{
    do_i_need_to_answer_question_now, update_daily_progress,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub no: i64,
    pub level: i64,
    pub question: String,
    #[serde(rename = "correct_answer")]
    pub correct_answer: i64,
    pub answers: Vec<String>,
    #[serde(rename = "point_reward")]
    pub point_reward: i64,
    pub tags: Vec<String>,
    pub hint: String,
    pub reference: Vec<String>,
    #[serde(rename = "correct_response")]
    pub correct_response: String,
    #[serde(rename = "wrong_response")]
    pub wrong_response: String,
}

pub fn learn_command_handler(force: bool) {
    if force || do_i_need_to_answer_question_now() {
        // vocabulary for identifiers
        let vocabulrary: [char; 26] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];

        // Let's override the `j` and `k` keys for navigation
        let _siv = cursive::default();
        // Read the list of questions from separate file, and fill the view with it.
        // (We include the file at compile-time to avoid runtime read errors.)
        let json_content = include_str!("assets/questions.json");
        let questions: Vec<Question> = serde_json::from_str(json_content).unwrap();

        let random_number = rand::thread_rng().gen_range(0..questions.len());
        let random_question = questions[random_number].clone();
        let mut siv = cursive::default();

        let mut question_content = random_question.question.clone();
        question_content.push('\n');
        for (index, answer) in random_question.answers.iter().enumerate() {
            let ans = format!("\n{}. {}\n", vocabulrary[index], answer);
            question_content.push_str(ans.as_str());
        }

        let table_view = TextView::new(&question_content)
            .h_align(HAlign::Left)
            .scrollable();

        let mut answers_dialog = Dialog::around(table_view)
            .title("RLRN")
            .h_align(HAlign::Center);

        // add answers buttons
        for (index, _) in random_question.answers.iter().enumerate() {
            let question = random_question.clone();

            answers_dialog = answers_dialog.button(vocabulrary[index], move |s| {
                let answered_correctly = index as i64 == question.correct_answer;
                update_daily_progress(answered_correctly);

                if answered_correctly {
                    s.add_layer(
                        Dialog::info("Correct!")
                            .button("Next", |_| learn_command_handler(true))
                            .button("Quit", |_| std::process::exit(0)),
                    );
                } else {
                    let reference = question.reference[0].to_string();
                    s.add_layer(Dialog::info(format!("Wrong.\n\n{}", reference)));
                }
            });
        }

        answers_dialog = answers_dialog.button("Hint", move |s| {
            let correct_answer =
                random_question.answers[random_question.correct_answer as usize].clone();
            s.add_layer(Dialog::info(correct_answer));
        });

        siv.add_layer(answers_dialog);

        // Starts the event loop.
        siv.run();
    }
}
