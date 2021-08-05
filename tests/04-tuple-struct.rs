use iced::text_input;
use iced_focus::Focus;

#[derive(Default, Focus)]
struct Example(
    #[focus(enable)]
    text_input::State,
    #[focus(enable)]
    text_input::State
);

fn main() {
    let mut example = Example::default();
    assert!(!example.has_focus());
    assert!(!example.0.has_focus());
    assert!(!example.1.has_focus());

    assert_eq!(example.focus(iced_focus::Direction::Forwards), iced_focus::State::Kept);
    assert!(example.has_focus());
    assert!(example.0.has_focus());
    assert!(!example.1.has_focus());
    
    assert_eq!(example.focus(iced_focus::Direction::Forwards), iced_focus::State::Kept);
    assert!(example.has_focus());
    assert!(!example.0.has_focus());
    assert!(example.1.has_focus());
    
    assert_eq!(example.focus(iced_focus::Direction::Forwards), iced_focus::State::Returned);
    assert!(!example.has_focus());
    assert!(!example.0.has_focus());
    assert!(!example.1.has_focus());
}