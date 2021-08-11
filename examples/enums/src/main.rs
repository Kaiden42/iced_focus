use iced::{
    button, executor, text_input, Application, Button, Column, Container, Element, Length, Row,
    Sandbox, Settings, Subscription, Text, TextInput,
};
use iced_focus::Focus;
//use iced_focus_derive::Focus;

#[derive(Debug, Focus)]
struct EnumsExample {
    button_none: button::State,
    button_one: button::State,
    button_two: button::State,

    #[focus(enable)]
    enum_state: EnumState,
}

#[derive(Clone, Debug)]
pub enum Message {
    Show(EnumVariant),
    One(String),
    Two(String),
    Focus(iced_focus::Direction),
}

fn main() -> iced::Result {
    EnumsExample::run(Settings::default())
}

impl Application for EnumsExample {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            EnumsExample {
                button_none: button::State::new(),
                button_one: button::State::new(),
                button_two: button::State::new(),
                enum_state: EnumState::None,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        "Enum Focus Example".into()
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match message {
            Message::Show(variant) => match variant {
                EnumVariant::None => self.enum_state = EnumState::None,
                EnumVariant::One => self.enum_state = EnumState::one(),
                EnumVariant::Two => self.enum_state = EnumState::two(),
            },
            Message::One(value) => match self.enum_state {
                EnumState::None => unreachable!(),
                EnumState::One { ref mut one, .. } => *one = value,
                EnumState::Two { ref mut one, .. } => *one = value,
            },
            Message::Two(value) => match self.enum_state {
                EnumState::None => unreachable!(),
                EnumState::One { .. } => unreachable!(),
                EnumState::Two { ref mut two, .. } => *two = value,
            },
            Message::Focus(direction) => {
                let _ = self.focus(direction);
            }
        }

        iced::Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events_with(|event, _status| {
            if let iced_native::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key_code: iced_native::keyboard::KeyCode::Tab,
                modifiers,
            }) = event
            {
                Some(Message::Focus(if modifiers.shift {
                    iced_focus::Direction::Backwards
                } else {
                    iced_focus::Direction::Forwards
                }))
            } else {
                None
            }
        })
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = self.enum_state.view();

        Container::new(
            Column::new()
                .push(
                    Row::new()
                        .spacing(5)
                        .push(
                            Button::new(&mut self.button_none, Text::new("None"))
                                .on_press(Message::Show(EnumVariant::None)),
                        )
                        .push(
                            Button::new(&mut self.button_one, Text::new("One"))
                                .on_press(Message::Show(EnumVariant::One)),
                        )
                        .push(
                            Button::new(&mut self.button_two, Text::new("Two"))
                                .on_press(Message::Show(EnumVariant::Two)),
                        ),
                )
                .push(content),
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EnumVariant {
    None,
    One,
    Two,
}

#[derive(Debug, Focus)]
//#[derive(Debug)]
enum EnumState {
    None,

    One {
        one: String,
        #[focus(enable)]
        text_input_one: text_input::State,
    },

    Two {
        one: String,
        #[focus(enable)]
        text_input_one: text_input::State,
        two: String,
        #[focus(enable)]
        text_input_two: text_input::State,
    },
}

impl EnumState {
    pub fn one() -> Self {
        Self::One {
            one: "One".into(),
            text_input_one: text_input::State::new(),
        }
    }

    pub fn two() -> Self {
        Self::Two {
            one: "One".into(),
            text_input_one: text_input::State::new(),
            two: "Two".into(),
            text_input_two: text_input::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<'_, Message> {
        match self {
            EnumState::None => Column::new(),
            EnumState::One {
                one,
                text_input_one,
            } => Column::new()
                .push(TextInput::new(text_input_one, "One", one, Message::One).padding(5)),
            EnumState::Two {
                one,
                text_input_one,
                two,
                text_input_two,
            } => Column::new()
                .push(TextInput::new(text_input_one, "One", one, Message::One).padding(5))
                .push(TextInput::new(text_input_two, "One", two, Message::Two).padding(5)),
        }
        .max_width(600)
        .padding(5)
        .spacing(5)
        .into()
    }
}

//impl iced_focus::Focus for EnumState {
//    fn focus(&mut self, direction: iced_focus::Direction) -> iced_focus::State {
//        match self {
//            Self::None => {
//                let mut fields: std::vec::Vec<&mut dyn iced_focus::Focus> =
//                    std::vec::Vec::with_capacity(0usize);
//                fields.focus(direction)
//            }
//            Self::One { text_input_one, .. } => {
//                let mut fields: std::vec::Vec<&mut dyn iced_focus::Focus> =
//                    std::vec::Vec::with_capacity(1usize);
//                fields.push(text_input_one);
//                fields.focus(direction)
//            }
//            Self::Two {
//                text_input_one,
//                text_input_two,
//                ..
//            } => {
//                let mut fields: std::vec::Vec<&mut dyn iced_focus::Focus> =
//                    std::vec::Vec::with_capacity(2usize);
//                fields.push(text_input_one);
//                fields.push(text_input_two);
//                fields.focus(direction)
//            }
//        }
//    }
//    fn has_focus(&self) -> bool {
//        match self {
//            Self::None => false,
//            Self::One { text_input_one, .. } => text_input_one.has_focus() || false,
//            Self::Two {
//                text_input_one,
//                text_input_two,
//                ..
//            } => text_input_one.has_focus() || text_input_two.has_focus() || false,
//        }
//    }
//}
