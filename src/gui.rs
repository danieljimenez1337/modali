use crate::input;
use crate::parser;
use crate::parser::{Action, WhichTreeKind, WhichTreeNode};
use crate::util;
use iced::widget::{Column, container, row, text};
use iced::{Alignment, Color, Element, Event, Length, Task as Command, Theme, event};
use iced_layershell::Application;
use iced_layershell::to_layer_message;

pub struct Modali {
    pub buffer: String,
    pub whichtree: WhichTreeNode,
}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    IcedEvent(Event),
}

impl Application for Modali {
    type Message = Message;
    type Flags = super::Args;
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(flags: super::Args) -> (Self, Command<Message>) {
        let whichtree = {
            let contents = util::load_keybindings(flags.input).unwrap();
            let actions: Vec<Action> = serde_json::from_str(&contents).unwrap();
            parser::actions_to_tree(&actions)
        };

        (
            Self {
                buffer: String::new(),
                whichtree,
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("Modali")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(Message::IcedEvent)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IcedEvent(Event::Keyboard(key_event)) => {
                input::handle_keyboard_input(self, key_event)
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        const MAX_ITEMS_PER_COLUMN: usize = 10;
        const FONT_SIZE: u16 = 22;

        let children = match parser::search_which_tree(&self.whichtree, &self.buffer) {
            Some(x) => match &x.kind {
                WhichTreeKind::Command(_) => Vec::new(),
                WhichTreeKind::Children(x) => x.clone(),
            },
            None => Vec::new(),
        };

        let num_columns = if children.is_empty() {
            1
        } else {
            (children.len() + MAX_ITEMS_PER_COLUMN - 1) / MAX_ITEMS_PER_COLUMN
        };

        let mut rows = Vec::new();
        for i in 0..num_columns {
            let column_items = children.iter()
                .skip(i * MAX_ITEMS_PER_COLUMN)
                .take(MAX_ITEMS_PER_COLUMN)
                .map(|tree_node| text(tree_node.label.clone()).size(FONT_SIZE))
                .fold(Column::new(), |column, item| column.push(item));

            rows.push(column_items);
        }

        let main_row = rows.into_iter().fold(row!().spacing(40), |row, column| row.push(column));

        container(main_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(main_container_style())
            .align_x(Alignment::Start)
            .align_y(Alignment::Start)
            .padding(40)
            .into()
    }

    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        use iced_layershell::Appearance;
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }
}

fn main_container_style<'a>() -> iced::widget::container::StyleFn<'a, Theme> {
    Box::new(|_| iced::widget::container::Style {
        border: iced::Border {
            color: Color::from_rgb8(0, 0, 0),
            width: 2.0,
            radius: iced::border::Radius::new(10.0),
        },
        background: Some(iced::Background::Color(Color::from_rgba8(22, 22, 29, 0.9))),
        text_color: Some(Color::from_rgb8(220, 215, 186)),
        ..Default::default()
    })
}
