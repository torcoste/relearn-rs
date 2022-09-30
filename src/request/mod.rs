use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView};

pub fn request_command_handler(_force: bool) {
    let mut select = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center)
        // Use keyboard to jump to the pressed letters
        .autojump();

    let mut siv = cursive::default();
    // Read the list of cities from separate file, and fill the view with it.
    // (We include the file at compile-time to avoid runtime read errors.)
    let content = include_str!("assets/cities.txt");
    select.add_all_str(content.lines());
    let answer = "berlin";
    // let a = AnswerHandler::new(answer.to_string());
    // Sets the callback for when "Enter" is pressed.
    select.set_on_submit(move |s, item: &str| {
        if item == answer {
            s.pop_layer();
            s.add_layer(Dialog::info("Correct!"));
        } else {
            s.add_layer(Dialog::info("Wrong!"));
        }
    });

    // Let's override the `j` and `k` keys for navigation
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
    siv.add_layer(Dialog::around(select.scrollable()).title("Where are you from?"));

    siv.run();
}
