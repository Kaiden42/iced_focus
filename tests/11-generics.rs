use iced::text_input;
use iced_focus::Focus;

#[derive(Default, Focus)]
#[allow(unused)]
struct ExampleType<T: Default> {
    text: String,
    #[focus(enable)]
    text_input: text_input::State,
    generic: T,
}

#[derive(Focus)]
#[allow(unused)]
struct ExampleLifetime<'a, 'b: 'a> {
    text_one: &'a str,
    #[focus(enable)]
    text_input_one: text_input::State,
    text_two: &'b str,
    #[focus(enable)]
    text_input_two: text_input::State,
}

#[derive(Focus)]
#[allow(unused)]
struct ExampleConst<const N: usize> {
    texts: [String; N],
    #[focus(enable)]
    text_inputs: [text_input::State; N],
}

#[derive(Focus)]
#[allow(unused)]
struct Example<'a, T, const N: usize> {
    text: &'a str,
    #[focus(enable)]
    text_input: text_input::State,
    element: T,
    bytes: [u8; N],
}

#[derive(Focus)]
#[allow(unused)]
enum ExampleEnum<'a, 'b: 'a, T: Default, const N: usize> {
    Lifetime {
        text_one: &'a str,
        text_two: &'b str,
        #[focus(enable)]
        text_input: text_input::State,
    },

    Ty {
        element: T,
        #[focus(enable)]
        text_input: text_input::State,
    },

    Con {
        bytes: [u8; N],
        #[focus(enable)]
        text_input: text_input::State,
    },
}

fn main() {}
