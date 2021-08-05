use iced::text_input;
use iced_focus::Focus;

#[derive(Focus)]
enum Example {
    One(
        #[focus(enable)]
        text_input::State
    ),
    Two(
        #[focus(enable)]
        text_input::State,
        #[focus(enable)]
        text_input::State,
    ),
}

fn main() {
    let mut example = Example::One(text_input::State::new());
    assert!(!example.has_focus());
    if let Example::One(ref t_0) = example {
        assert!(!t_0.has_focus());
    }

    assert_eq!(example.focus(iced_focus::Direction::Forwards), iced_focus::State::Kept);
    assert!(example.has_focus());
    if let Example::One(ref t_0) = example {
        assert!(t_0.has_focus());
    }

    assert_eq!(example.focus(iced_focus::Direction::Forwards), iced_focus::State::Returned);
    assert!(!example.has_focus());
    if let Example::One(ref t_0) = example {
        assert!(!t_0.has_focus());
    }

    example = Example::Two(text_input::State::new(), text_input::State::new());
    assert!(!example.has_focus());
    if let Example::Two(ref t_0, ref t_1) = example {
        assert!(!t_0.has_focus());
        assert!(!t_1.has_focus());
    }

    assert_eq!(example.focus(iced_focus::Direction::Forwards), iced_focus::State::Kept);
    assert!(example.has_focus());
    if let Example::Two(ref t_0, ref t_1) = example {
        assert!(t_0.has_focus());
        assert!(!t_1.has_focus());
    }

    assert_eq!(example.focus(iced_focus::Direction::Forwards), iced_focus::State::Kept);
    assert!(example.has_focus());
    if let Example::Two(ref t_0, ref t_1) = example {
        assert!(!t_0.has_focus());
        assert!(t_1.has_focus());
    }

    assert_eq!(example.focus(iced_focus::Direction::Forwards), iced_focus::State::Returned);
    assert!(!example.has_focus());
    if let Example::Two(ref t_0, ref t_1) = example {
        assert!(!t_0.has_focus());
        assert!(!t_1.has_focus());
    }
}