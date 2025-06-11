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
                if let Some(start) = super::START.get() {
                    println!("Elapsed: {:.2?} : {:?}", start.elapsed(), key_event);
                } else {
                    println!("START not initialized.");
                }
                input::handle_keyboard_input(self, key_event)
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut col0 = Column::new();
        let mut col1 = Column::new();
        let mut col2 = Column::new();

        let children = match parser::search_which_tree(&self.whichtree, &self.buffer) {
            Some(x) => match &x.kind {
                WhichTreeKind::Command(_) => &Vec::new(),
                WhichTreeKind::Children(x) => &x.clone(),
            },
            None => &Vec::new(),
        };

        for (i, tree_node) in children.iter().enumerate() {
            let col_num = i % 3;
            let label = tree_node.label.clone();
            match col_num {
                0 => {
                    col0 = col0.push(text(label).size(22));
                }

                1 => {
                    col1 = col1.push(text(label).size(22));
                }
                2 => {
                    col2 = col2.push(text(label).size(22));
                }
                _ => panic!(),
            }
        }

        let main = row![col0, col1, col2].spacing(40);

        container(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(main_container_style())
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
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
