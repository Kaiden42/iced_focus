use iced::text_input;
use iced_focus::Focus;

#[derive(Focus)]
struct Example {
    #[focus(enable)]
    text_input: Option<text_input::State>,
}

fn main() {
    let mut example = Example { text_input: None };
    assert!(!example.has_focus());
    assert!(!example.text_input.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Ignored
    );
    assert!(!example.has_focus());
    assert!(!example.text_input.has_focus());

    let mut example = Example {
        text_input: Some(text_input::State::new()),
    };

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(example.text_input.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Returned
    );
    assert!(!example.has_focus());
    assert!(!example.text_input.has_focus());
}
