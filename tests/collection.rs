#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-struct.rs");
    t.pass("tests/02-enum-unit.rs");
    t.pass("tests/03-enum-struct.rs");
    t.pass("tests/04-tuple-struct.rs");
    t.pass("tests/05-tuple-enum.rs");
    t.pass("tests/06-enable-with.rs");
}

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

#[derive(Debug, Focus)]
struct ExampleTupleStruct(
    #[focus(enable)]
    text_input::State,
    bool,
    #[focus(enable = "self.enable_input_two")]
    text_input::State,
);

impl ExampleTupleStruct {
    fn enable_input_two(&self) -> bool {
        self.1
    }
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
        #[focus(enable)]
        text_input::State,
        bool,
        #[focus(enable = "self.enable_input_two")]
        text_input::State,
    )
}

impl ExampleEnum {
    fn enable_input_two(&self) -> bool {
        match self {
            ExampleEnum::First { enable_two, .. }
            | ExampleEnum::Second(_, enable_two, _) => *enable_two,
        }
    }
}