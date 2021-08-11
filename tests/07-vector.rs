use iced::text_input;
use iced_focus::Focus;

#[derive(Focus)]
struct Example {
    #[focus(enable)]
    text_input_first: text_input::State,
    #[focus(enable)]
    text_inputs: Vec<text_input::State>,
    #[focus(enable)]
    text_input_last: text_input::State,
}

fn test_forwards() {
    let mut example = Example {
        text_input_first: text_input::State::new(),
        text_inputs: Vec::new(),
        text_input_last: text_input::State::new(),
    };
    assert!(!example.has_focus());
    assert!(!example.text_input_first.has_focus());
    assert!(!example.text_inputs.has_focus());
    assert!(!example.text_input_last.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(example.text_input_first.has_focus());
    assert!(!example.text_inputs.has_focus());
    assert!(!example.text_input_last.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(!example.text_input_first.has_focus());
    assert!(!example.text_inputs.has_focus());
    assert!(example.text_input_last.has_focus());

    let mut example = Example {
        text_input_first: text_input::State::new(),
        text_inputs: std::iter::repeat(text_input::State::new())
            .take(10)
            .collect(),
        text_input_last: text_input::State::new(),
    };

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(example.text_input_first.has_focus());
    assert!(!example.text_inputs.has_focus());
    assert!(!example.text_input_last.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(!example.text_input_first.has_focus());
    assert!(example.text_inputs.has_focus());
    assert!(!example.text_input_last.has_focus());

    let range: Vec<usize> = (0..example.text_inputs.len()).into_iter().collect();
    for window in range.windows(2) {
        assert_eq!(
            example.focus(iced_focus::Direction::Forwards),
            iced_focus::State::Kept
        );
        assert!(example.has_focus());
        assert!(!example.text_input_first.has_focus());
        assert!(!example.text_input_last.has_focus());

        assert!(example.text_inputs.has_focus());
        assert!(!example.text_inputs[window[0]].has_focus());
        assert!(example.text_inputs[window[1]].has_focus());
    }

    assert!(example.has_focus());
    assert!(!example.text_input_first.has_focus());
    assert!(example.text_inputs.has_focus());
    assert!(!example.text_input_last.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(!example.text_input_first.has_focus());
    assert!(!example.text_inputs.has_focus());
    assert!(example.text_input_last.has_focus());
}

fn test_backwards() {
    let mut example = Example {
        text_input_first: text_input::State::new(),
        text_inputs: Vec::new(),
        text_input_last: text_input::State::new(),
    };
    assert!(!example.has_focus());
    assert!(!example.text_input_last.has_focus());
    assert!(!example.text_inputs.has_focus());
    assert!(!example.text_input_first.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Backwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(example.text_input_last.has_focus());
    assert!(!example.text_inputs.has_focus());
    assert!(!example.text_input_first.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Backwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(!example.text_input_last.has_focus());
    assert!(!example.text_inputs.has_focus());
    assert!(example.text_input_first.has_focus());

    let mut example = Example {
        text_input_first: text_input::State::new(),
        text_inputs: std::iter::repeat(text_input::State::new())
            .take(10)
            .collect(),
        text_input_last: text_input::State::new(),
    };

    assert_eq!(
        example.focus(iced_focus::Direction::Backwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(example.text_input_last.has_focus());
    assert!(!example.text_inputs.has_focus());
    assert!(!example.text_input_first.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Backwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(!example.text_input_last.has_focus());
    assert!(example.text_inputs.has_focus());
    assert!(!example.text_input_first.has_focus());

    let range: Vec<usize> = (0..example.text_inputs.len()).into_iter().rev().collect();
    for window in range.windows(2) {
        assert_eq!(
            example.focus(iced_focus::Direction::Backwards),
            iced_focus::State::Kept
        );
        assert!(example.has_focus());
        assert!(!example.text_input_last.has_focus());
        assert!(!example.text_input_first.has_focus());

        assert!(example.text_inputs.has_focus());
        assert!(!example.text_inputs[window[0]].has_focus());
        assert!(example.text_inputs[window[1]].has_focus());
    }

    assert!(example.has_focus());
    assert!(!example.text_input_last.has_focus());
    assert!(example.text_inputs.has_focus());
    assert!(!example.text_input_first.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Backwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(!example.text_input_last.has_focus());
    assert!(!example.text_inputs.has_focus());
    assert!(example.text_input_first.has_focus());
}

fn main() {
    test_forwards();
    test_backwards();
}
