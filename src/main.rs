use seed::{
    prelude::{
        web_sys::{Event, HtmlInputElement, KeyboardEvent},
        *,
    },
    *,
};
use statics::LOGO;

use crate::statics::{SHELL_PROMPT, SHELL_STATUS};

mod shell;
mod statics;

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.stream(streams::window_event(Ev::KeyDown, |event| {
                        Msg::KeyPressed(event.unchecked_into())
                    }));

    Model {
        buffer: LOGO.to_owned(),
        input: String::new(),
        cursor_pos: 0usize,
        cursor_buffer: "█".to_owned(),
        input_element: Default::default(),
    }
}

struct Model {
    buffer: String,
    input: String,
    cursor_pos: usize,
    cursor_buffer: String,
    input_element: ElRef<HtmlInputElement>,
}

#[derive(Clone)]
enum Msg {
    KeyPressed(KeyboardEvent),
    WindowClicked(Event),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::KeyPressed(key_ev) => match key_ev.key_code() {
            // backspace
            8 => {
                if model.cursor_pos.clone() > 0usize {
                    model.input.remove(model.cursor_pos - 1);
                    model.cursor_pos -= 1;
                    _ = model.cursor_buffer.remove(0);
                }
            }

            // enter
            13 => {
                model.buffer = format!(
                    "{}\n{}\n{}{}\n{}",
                    model.buffer,
                    SHELL_STATUS,
                    SHELL_PROMPT,
                    &model.input,
                    &shell::execute(&model.input)
                );
                model.input = String::new();
                model.cursor_pos = 0usize;
                model.cursor_buffer = "█".to_owned();
            }

            // left
            37 => {
                if model.cursor_pos > 0 {
                    model.cursor_buffer.remove(0);
                    model.cursor_pos -= 1;
                }
            }

            // right
            39 => {
                if model.cursor_pos < model.input.len() {
                    model.cursor_buffer.insert(0, ' ');
                    model.cursor_pos += 1;
                }
            }

            // any
            _ => {
                let key = key_ev.key();
                let key_len = key.len();
                model.input.insert_str(model.cursor_pos, &key);
                model.cursor_pos += key_len;
                for _ in 0..key_len {
                    model.cursor_buffer.insert(0, ' ')
                }
            }
        },

        Msg::WindowClicked(_) => {}
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["console crt"],
        div![
            C!["max-content"],
            attrs! {At::TabIndex => "0"},
            pre![raw!(&model.buffer)],
            pre![raw!(SHELL_STATUS)],
            div![
                C!["inputbox"],
                pre![raw!(SHELL_PROMPT)],
                div![
                    C!["inputholder"],
                    pre![&model.input, C!["input"]],
                    pre![&model.cursor_buffer, C!["cursor"]]
                ],
                input![C!["inexistant"], el_ref(&model.input_element)]
            ],
        ],
        ev(Ev::Click, Msg::WindowClicked)
    ]
}

pub fn main() {
    App::start("app", init, update, view);
}
