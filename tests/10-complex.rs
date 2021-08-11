use iced::text_input;
use iced_focus::Focus;

#[allow(unused, dead_code)]
#[derive(Default, Focus)]
struct Example {
    one: String,
    #[focus(enable)]
    text_input_one: text_input::State,

    #[focus(enable)]
    example_a: ExampleA,

    #[focus(enable)]
    example_b: ExampleB,
}

#[allow(unused, dead_code)]
#[derive(Focus)]
struct ExampleA {
    first: String,
    #[focus(enable)]
    text_input_first: text_input::State,

    #[focus(enable)]
    text_inputs: Vec<ExampleC>,

    last: String,
    #[focus(enable)]
    text_input_last: text_input::State,
}

impl Default for ExampleA {
    fn default() -> Self {
        Self {
            first: String::default(),
            text_input_first: text_input::State::default(),
            text_inputs: std::iter::repeat(ExampleC::default()).take(10).collect(),
            last: String::default(),
            text_input_last: text_input::State::default(),
        }
    }
}

#[allow(unused, dead_code)]
#[derive(Clone, Default, Focus)]
struct ExampleC {
    text: String,
    #[focus(enable)]
    text_inputs: text_input::State,
}

#[allow(unused, dead_code)]
#[derive(Focus)]
struct ExampleB {
    text: String,
    enable_text_input: bool,
    #[focus(enable = "self.enable_text_input")]
    text_input: text_input::State,
}

impl ExampleB {
    fn enable_text_input(&self) -> bool {
        self.enable_text_input
    }
}

impl Default for ExampleB {
    fn default() -> Self {
        Self {
            text: String::default(),
            enable_text_input: true,
            text_input: text_input::State::default(),
        }
    }
}

fn test_forwards() {
    let mut example = Example::default();
    assert!(!example.has_focus());
    assert!(!example.text_input_one.has_focus());
    assert!(!example.example_a.has_focus());
    assert!(!example.example_b.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(example.text_input_one.has_focus());
    assert!(!example.example_a.has_focus());
    assert!(!example.example_b.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(!example.text_input_one.has_focus());
    assert!(example.example_a.has_focus());
    assert!(!example.example_b.has_focus());

    assert!(example.example_a.text_input_first.has_focus());
    assert!(!example.example_a.text_inputs.has_focus());
    assert!(!example.example_a.text_input_last.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(!example.example_a.text_input_first.has_focus());
    assert!(example.example_a.text_inputs.has_focus());
    assert!(!example.example_a.text_input_last.has_focus());

    let range: Vec<usize> = (0..example.example_a.text_inputs.len())
        .into_iter()
        .collect();
    for window in range.windows(2) {
        assert_eq!(
            example.focus(iced_focus::Direction::Forwards),
            iced_focus::State::Kept
        );
        assert!(!example.text_input_one.has_focus());
        assert!(example.example_a.has_focus());
        assert!(!example.example_b.has_focus());

        assert!(example.example_a.text_inputs.has_focus());
        assert!(!example.example_a.text_inputs[window[0]].has_focus());
        assert!(example.example_a.text_inputs[window[1]].has_focus());
    }

    assert!(example.has_focus());
    assert!(!example.text_input_one.has_focus());
    assert!(example.example_a.has_focus());
    assert!(!example.example_b.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.example_a.has_focus());
    assert!(!example.example_b.has_focus());
    assert!(example.example_a.text_input_last.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(!example.example_a.has_focus());
    assert!(example.example_b.has_focus());
    assert!(example.example_b.text_input.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Returned
    );
    assert!(!example.has_focus());
    assert!(!example.text_input_one.has_focus());
    assert!(!example.example_a.has_focus());
    assert!(!example.example_b.has_focus());
}

fn main() {
    test_forwards();
    //test_backwards();
}
