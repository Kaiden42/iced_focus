use iced::text_input;
use iced_focus::Focus;

#[derive(Default, Focus)]
struct Example {
    #[focus(enable)]
    text_inputs: [text_input::State; 10],
}

fn test_forwards() {
    let mut example = Example::default();
    assert!(!example.has_focus());
    assert!(!example.text_inputs.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(example.text_inputs.has_focus());

    let range: Vec<usize> = (0..example.text_inputs.len()).into_iter().collect();
    for window in range.windows(2) {
        assert_eq!(
            example.focus(iced_focus::Direction::Forwards),
            iced_focus::State::Kept
        );
        assert!(example.has_focus());
        assert!(example.text_inputs.has_focus());
        assert!(!example.text_inputs[window[0]].has_focus());
        assert!(example.text_inputs[window[1]].has_focus());
    }

    assert!(example.has_focus());
    assert!(example.text_inputs.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Returned
    );
    assert!(!example.has_focus());
    assert!(!example.text_inputs.has_focus());
}

fn test_backwards() {
    let mut example = Example::default();
    assert!(!example.has_focus());
    assert!(!example.text_inputs.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Backwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(example.text_inputs.has_focus());

    let range: Vec<usize> = (0..example.text_inputs.len()).into_iter().rev().collect();
    for window in range.windows(2) {
        assert_eq!(
            example.focus(iced_focus::Direction::Backwards),
            iced_focus::State::Kept
        );
        assert!(example.has_focus());
        assert!(example.text_inputs.has_focus());
        assert!(!example.text_inputs[window[0]].has_focus());
        assert!(example.text_inputs[window[1]].has_focus());
    }

    assert!(example.has_focus());
    assert!(example.text_inputs.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Backwards),
        iced_focus::State::Returned
    );
    assert!(!example.has_focus());
    assert!(!example.text_inputs.has_focus());
}

fn main() {
    test_forwards();
    test_backwards();
}
