use crate::gui;
use iced_runtime::Action;

use iced::{
    Task as Command,
    keyboard::{self, Modifiers},
};

use crate::{
    parser::{self, WhichTreeKind},
    util,
};

pub fn handle_keyboard_input(
    state: &mut gui::Modali,
    key_event: keyboard::Event,
) -> Command<gui::Message> {
    match key_event {
        iced::keyboard::Event::KeyReleased { key, modifiers, .. } => match key {
            keyboard::Key::Named(iced::keyboard::key::Named::Escape) => {
                iced_runtime::task::effect(Action::Exit)
            }
            keyboard::Key::Named(iced::keyboard::key::Named::Backspace) => {
                state.buffer.pop();
                Command::none()
            }
            keyboard::Key::Character(c) => {
                let key = match modifiers {
                    Modifiers::SHIFT => c.to_uppercase(),
                    _ => c.to_string(),
                };

                state.buffer.push_str(&key);

                match parser::search_which_tree(&state.whichtree, &state.buffer) {
                    Some(x) => match &x.kind {
                        WhichTreeKind::Command(cmd) => {
                            util::run_command_detached(cmd).unwrap();
                            iced_runtime::task::effect(Action::Exit)
                        }
                        WhichTreeKind::Children(_) => Command::none(),
                    },
                    None => {
                        state.buffer.pop();
                        Command::none()
                    }
                }
            }
            _ => Command::none(),
        },
        _ => Command::none(),
    }
}
