use iced::text_input;
use iced_focus::Focus;

#[derive(Default, Focus)]
struct ExampleStruct {
    #[focus(enable)]
    text_input_one: text_input::State,
    enable_two: bool,
    #[focus(enable = "self.enable_input_two")]
    text_input_two: text_input::State,
}

impl ExampleStruct {
    fn enable_input_two(&self) -> bool {
        self.enable_two
    }
}

fn test_struct() {
    let mut example = ExampleStruct::default();
    assert!(!example.has_focus());
    assert!(!example.text_input_one.has_focus());
    assert!(!example.text_input_two.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(example.text_input_one.has_focus());
    assert!(!example.text_input_two.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Returned
    );
    assert!(!example.has_focus());
    assert!(!example.text_input_one.has_focus());
    assert!(!example.text_input_two.has_focus());

    let mut example = ExampleStruct {
        enable_two: true,
        ..ExampleStruct::default()
    };

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(example.text_input_one.has_focus());
    assert!(!example.text_input_two.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(!example.text_input_one.has_focus());
    assert!(example.text_input_two.has_focus());

    example.enable_two = false;
    assert!(!example.has_focus());
    assert!(example.text_input_two.has_focus()); // TODO: this should not be true!

    example.enable_two = true;
    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Returned
    );
    assert!(!example.has_focus());
    assert!(!example.text_input_one.has_focus());
    assert!(!example.text_input_two.has_focus());
}

#[derive(Default, Focus)]
struct ExampleTupleStruct(
    #[focus(enable)] text_input::State,
    bool,
    #[focus(enable = "self.enable_input_two")] text_input::State,
);

impl ExampleTupleStruct {
    fn enable_input_two(&self) -> bool {
        self.1
    }
}

fn test_tuple_struct() {
    let mut example = ExampleTupleStruct::default();
    assert!(!example.has_focus());
    assert!(!example.0.has_focus());
    assert!(!example.2.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(example.0.has_focus());
    assert!(!example.2.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Returned
    );
    assert!(!example.has_focus());
    assert!(!example.0.has_focus());
    assert!(!example.2.has_focus());

    let mut example = ExampleTupleStruct {
        1: true,
        ..ExampleTupleStruct::default()
    };

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(example.0.has_focus());
    assert!(!example.2.has_focus());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    assert!(!example.0.has_focus());
    assert!(example.2.has_focus());

    example.1 = false;
    assert!(!example.has_focus());
    assert!(example.2.has_focus()); // TODO: this should not be true!

    example.1 = true;
    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Returned
    );
    assert!(!example.has_focus());
    assert!(!example.0.has_focus());
    assert!(!example.2.has_focus());
}

#[derive(Focus)]
enum ExampleEnum {
    First {
        #[focus(enable)]
        text_input_one: text_input::State,
        enable_two: bool,
        #[focus(enable = "self.enable_input_two")]
        text_input_two: text_input::State,
    },
    Second(
        #[focus(enable)] text_input::State,
        bool,
        #[focus(enable = "self.enable_input_two")] text_input::State,
    ),
}

impl ExampleEnum {
    fn enable_input_two(&self) -> bool {
        match self {
            ExampleEnum::First { enable_two, .. } | ExampleEnum::Second(_, enable_two, _) => {
                *enable_two
            }
        }
    }
}

fn test_enum_first() {
    let mut example = ExampleEnum::First {
        text_input_one: text_input::State::new(),
        enable_two: false,
        text_input_two: text_input::State::new(),
    };
    assert!(!example.has_focus());
    if let ExampleEnum::First {
        ref text_input_one,
        ref text_input_two,
        ..
    } = example
    {
        assert!(!text_input_one.has_focus());
        assert!(!text_input_two.has_focus());
    }

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    if let ExampleEnum::First {
        ref text_input_one,
        ref text_input_two,
        ..
    } = example
    {
        assert!(text_input_one.has_focus());
        assert!(!text_input_two.has_focus());
    }

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Returned
    );
    assert!(!example.has_focus());
    if let ExampleEnum::First {
        ref text_input_one,
        ref text_input_two,
        ..
    } = example
    {
        assert!(!text_input_one.has_focus());
        assert!(!text_input_two.has_focus());
    }

    let mut example = ExampleEnum::First {
        text_input_one: text_input::State::new(),
        enable_two: true,
        text_input_two: text_input::State::new(),
    };

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    if let ExampleEnum::First {
        ref text_input_one,
        ref text_input_two,
        ..
    } = example
    {
        assert!(text_input_one.has_focus());
        assert!(!text_input_two.has_focus());
    }

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    if let ExampleEnum::First {
        ref text_input_one,
        ref text_input_two,
        ..
    } = example
    {
        assert!(!text_input_one.has_focus());
        assert!(text_input_two.has_focus());
    }

    if let ExampleEnum::First {
        ref mut enable_two, ..
    } = example
    {
        *enable_two = false;
    }
    assert!(!example.has_focus());

    if let ExampleEnum::First {
        ref mut enable_two, ..
    } = example
    {
        *enable_two = true;
    }
    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Returned
    );
    assert!(!example.has_focus());
    if let ExampleEnum::First {
        ref text_input_one,
        ref text_input_two,
        ..
    } = example
    {
        assert!(!text_input_one.has_focus());
        assert!(!text_input_two.has_focus());
    }
}

fn test_enum_second() {
    let mut example =
        ExampleEnum::Second(text_input::State::new(), false, text_input::State::new());
    assert!(!example.has_focus());
    if let ExampleEnum::Second(ref text_input_one, _, ref text_input_two) = example {
        assert!(!text_input_one.has_focus());
        assert!(!text_input_two.has_focus());
    }

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    if let ExampleEnum::Second(ref text_input_one, _, ref text_input_two) = example {
        assert!(text_input_one.has_focus());
        assert!(!text_input_two.has_focus());
    }

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Returned
    );
    assert!(!example.has_focus());
    if let ExampleEnum::Second(ref text_input_one, _, ref text_input_two) = example {
        assert!(!text_input_one.has_focus());
        assert!(!text_input_two.has_focus());
    }

    let mut example = ExampleEnum::Second(text_input::State::new(), true, text_input::State::new());

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    if let ExampleEnum::Second(ref text_input_one, _, ref text_input_two) = example {
        assert!(text_input_one.has_focus());
        assert!(!text_input_two.has_focus());
    }

    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Kept
    );
    assert!(example.has_focus());
    if let ExampleEnum::Second(ref text_input_one, _, ref text_input_two) = example {
        assert!(!text_input_one.has_focus());
        assert!(text_input_two.has_focus());
    }

    if let ExampleEnum::Second(_, ref mut enable_two, _) = example {
        *enable_two = false;
    }
    assert!(!example.has_focus());

    if let ExampleEnum::Second(_, ref mut enable_two, _) = example {
        *enable_two = true;
    }
    assert_eq!(
        example.focus(iced_focus::Direction::Forwards),
        iced_focus::State::Returned
    );
    assert!(!example.has_focus());
    if let ExampleEnum::Second(ref text_input_one, _, ref text_input_two) = example {
        assert!(!text_input_one.has_focus());
        assert!(!text_input_two.has_focus());
    }
}

fn main() {
    test_struct();
    test_tuple_struct();
    test_enum_first();
    test_enum_second();
}
