use iced::{
    Task as Command,
    keyboard::{self, Modifiers},
};

pub fn handle_keyboard_input(
    state: &mut super::Modali,
    key_event: keyboard::Event,
) -> Command<super::Message> {
    match key_event {
        iced::keyboard::Event::KeyReleased {
            key,
            location,
            modifiers,
        } => match key {
            keyboard::Key::Named(iced::keyboard::key::Named::Escape) => iced::exit(),
            keyboard::Key::Named(iced::keyboard::key::Named::Backspace) => {
                state.state.pop();
                Command::none()
            }
            keyboard::Key::Character(c) => {
                todo!()
            }
            _ => Command::none(),
        },
        _ => Command::none(),
    }
}

// fn combine_char_modifier(c: SmolStr, modifiers: Modifiers) -> String {
//     todo!()
// }
