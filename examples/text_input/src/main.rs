use iced::{
    executor, text_input, Application, Column, Container, Length, Settings, Subscription, TextInput,
};
use iced_focus::Focus;
//use iced_focus_derive::Focus;

#[derive(Default, Focus)]
struct TextInputFocusExample {
    one: String,
    #[focus(enable)]
    text_input_one: text_input::State,

    two: String,
    #[focus(enable)]
    text_input_two: text_input::State,

    three: String,
    #[focus(enable)]
    text_input_three: text_input::State,

    four: String,
    #[focus(enable)]
    text_input_four: text_input::State,
}

//impl Focus for TextInputFocusExample {
//    fn focus(&mut self, direction: iced_focus::Direction) -> iced_focus::State {
//        let mut vector: Vec<&mut dyn Focus> = vec![
//            &mut self.text_input_one,
//            &mut self.text_input_two,
//            &mut self.text_input_three,
//            &mut self.text_input_four,
//        ];
//
//        //vector.as_mut_slice().focus(direction)
//        vector.focus(direction)
//    }
//
//    fn has_focus(&self) -> bool {
//        false
//    }
//}

#[derive(Clone, Debug)]
enum Message {
    One(String),
    Two(String),
    Three(String),
    Four(String),
    Focus(iced_focus::Direction),
}

fn main() -> iced::Result {
    TextInputFocusExample::run(Settings::default())
}

impl Application for TextInputFocusExample {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (TextInputFocusExample::default(), iced::Command::none())
    }

    fn title(&self) -> String {
        "TextInput Focus Example".into()
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match message {
            Message::One(value) => self.one = value,
            Message::Two(value) => self.two = value,
            Message::Three(value) => self.three = value,
            Message::Four(value) => self.four = value,

            // Add this:
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

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Container::new(
            Column::new()
                .max_width(600)
                .spacing(5)
                .push(
                    TextInput::new(&mut self.text_input_one, "One", &self.one, Message::One)
                        .padding(5),
                )
                .push(
                    TextInput::new(&mut self.text_input_two, "Two", &self.two, Message::Two)
                        .padding(5),
                )
                .push(
                    TextInput::new(
                        &mut self.text_input_three,
                        "Three",
                        &self.three,
                        Message::Three,
                    )
                    .padding(5),
                )
                .push(
                    TextInput::new(&mut self.text_input_four, "Four", &self.four, Message::Four)
                        .padding(5),
                ),
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
