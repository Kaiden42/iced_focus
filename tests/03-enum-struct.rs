use iced::text_input;
use iced_focus::Focus;

#[derive(Focus)]
enum Example {
    None,
    One {
        #[focus(enable)]
        text_input_one: text_input::State
    },
    Two {
        #[focus(enable)]
        text_input_one: text_input::State,
        #[focus(enable)]
        text_input_two: text_input::State,
    }
}

fn main() {
    let mut example = Example::None;
    assert_eq!(example.focus(iced_focus::Direction::Forwards), iced_focus::State::Ignored);

    example = Example::One {
        text_input_one: text_input::State::new(),
    };
    assert_eq!(example.focus(iced_focus::Direction::Forwards), iced_focus::State::Kept);
    if let Example::One { text_input_one } = example {
        assert!(text_input_one.has_focus())
    }

    example = Example::Two {
        text_input_one: text_input::State::new(),
        text_input_two: text_input::State::new(),
    };
    assert_eq!(example.focus(iced_focus::Direction::Forwards), iced_focus::State::Kept);
    if let Example::Two { text_input_one, text_input_two } = example {
        assert!(text_input_one.has_focus());
        assert!(!text_input_two.has_focus());
    }
}