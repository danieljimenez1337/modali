use iced::{Task as Command, keyboard};

pub fn handle_keyboard_input(key_event: keyboard::Event) -> Command<super::Message> {
    match key_event {
        iced::keyboard::Event::KeyReleased {
            key,
            location,
            modifiers,
        } => match key {
            keyboard::Key::Named(iced::keyboard::key::Named::Escape) => iced::exit(),
            keyboard::Key::Character(c) => todo!(),
            _ => Command::none(),
        },
        _ => Command::none(),
    }
}
