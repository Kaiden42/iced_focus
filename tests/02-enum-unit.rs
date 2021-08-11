use iced_focus::Focus;

#[derive(Focus)]
enum Example {
    One,
    Two,
    Three,
}

fn main() {
    let mut example = Example::One;
    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Ignored
    );
    assert!(!example.has_focus());

    example = Example::Two;
    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Ignored
    );
    assert!(!example.has_focus());

    example = Example::Three;
    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Ignored
    );
    assert!(!example.has_focus());
}
