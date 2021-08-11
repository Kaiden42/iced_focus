# iced_focus

This crates adds a procedural macro to derive a focus chain for your application state. This allows the user of your `iced` application to swap the focus between the input fields of your user interface using `Tab` and `Shift+Tab`. The order of the focus chain will be derived of the order of the fields of your state. The crate `iced_focus` serves as a workaround until iced provides it's own focusing.


## How to use

Add `iced_focus` to your dependencies:
```toml
[dependencies]
iced_focus = { version = "0.1.0", features = ["derive"] }
iced = "0.3.0"
iced_native = "0.4.0"
```
_Note: you also need iced_native for the keyboard subscription_


Then derive the `Focus` trait for your State:
```rust
use iced::text_input;
use iced_focus::Focus;

#[derive(Focus)]
struct Example {
    text: String,
    #[focus(enable)]
    text_input: text_input::State,
}
```

As there is no way of knowing whether a field implements a specific trait, you will have to annotate each input field with `#focus(enable)` to add the field to the focus chain.

To handle the keyboard input of the user add an additional message like `Focus(iced_focus::Direction)` to your applications message definition:
```rust
enum Message {
    TextInput(String),
    // Add this:
    Focus(iced_focus::Direction),
}
```

Add this subscription to your application ...:
```rust
impl Application for Example {
    ...
    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events_with(|event, _status| {
            if let iced_native::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key_code: iced_native::keyboard::KeyCode::Tab,
                modifiers,
            }) = event
            {
                Some(Message::Focus(if modifiers.shift {
                    iced_focus::Direction::Backwards
                } else {
                    iced_focus::Direction::Forwards
                }))
            } else {
                None
            }
        })
    }
    ...
}
```

... and handle the focus message:
```rust
impl Application for Example {
    ...
    fn update(...) -> iced::Command<Self::Message> {
        match message {
            Message::TextInput(value) => self.text = value,

            // Add this:
            Message::Focus(direction) => {
                let _ = self.focus(direction);
            }
        }

        iced::Command::none()
    }
    ...
}
```

Done! Happy focusing! 🙂


You can specify whether the field will be added to the focus chain by providing a path to a method. The method must be of kind `Fn(&self) -> bool`.

```rust
struct Example {
    text: String,
    enable: bool,
    #[focus(enable = "self.enable_text_input")]
    text_input: text_input::State,
}

impl Example {
    fn enable_text_input(&self) -> bool {
        self.enable
    }
}
```

# What is supported by this crate?

Currently, only the `TextInput` widget is supported as it is the only widget that supports focusing. This crate only provides a linear focus chain based on the ordering of the fields. The actual position of the element on the window in unknown to the application state.

You can derive the Focus trait for structs, tuple structs and enums:

```rust
#[derive(Focus)]
struct ExampleTuple (
    String,
    #[focus(enable)]
    text_input::State,
);

#[derive(Focus)]
enum ExampleEnum {
    Unit,
    Named {
        text: String,
        #[focus(enable)]
        text_input: text_input::State,
    },
    Unnamed (
        String,
        #[focus(enable)]
        text_input::State,
    )
}
```

You can have vecs, arrays, options or nested structs:
```rust
#[derive(Focus)]
struct ExampleVec {
    #[focus(enable)]
    text_inputs: Vec<text_input::State>,
}

#[derive(Focus)]
struct ExampleArray {
    #[focus(enable)]
    text_inputs: [text_input::State; 10],
}

#[derive(Focus)]
struct ExampleOption {
    #[focus(enable)]
    text_input: Option<text_input::State>,
}

#[derive(Focus)]
struct Example {
    #[focus(enable)]
    vec: ExampleVec,

    #[focus(enable)]
    array: ExampleArray,

    #[focus(enable)]
    option: ExampleOption,
}
```
