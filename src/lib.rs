
// Thanks to serde: <https://github.com/serde-rs/serde/blob/master/serde/src/lib.rs>
#[cfg(feature = "derive")]
//#[allow(unused_imports)]
extern crate iced_focus_derive;
#[cfg(feature = "derive")]
#[doc(hidden)]
pub use iced_focus_derive::*;

pub trait Focus {
    fn focus(&mut self, direction: Direction) -> State;
    fn has_focus(&self) -> bool;

    // Ugly workaround... see: <https://stackoverflow.com/a/61654763>
    //fn as_dyn(&self) -> &dyn Focus;
    //fn as_dyn_mut(&mut self) -> &mut dyn Focus;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    /// When the input keeps the focus.
    Kept,
    /// When the input has returned the focus.    
    Returned,
    /// When the input has ignored the focus.
    Ignored,
}

impl Focus for iced::text_input::State {
    fn focus(&mut self, _direction: Direction) -> State {
        if self.is_focused() {
            self.unfocus();
            State::Returned
        } else {
            self.focus();
            State::Kept
        }
    }

    fn has_focus(&self) -> bool {
        self.is_focused()
    }
}

// TODO: reduce the amount of allocations needed.
impl<T: Focus> Focus for Vec<T> {
    fn focus(&mut self, direction: Direction) -> State {
        let mut vector: Vec<&mut dyn Focus> =
            self.iter_mut().map(|t| t as &mut dyn Focus).collect();
        //vector.as_mut_slice().focus(direction)
        vector.focus(direction)
    }

    fn has_focus(&self) -> bool {
        self.iter().any(|t| t.has_focus())
    }
}

impl Focus for Vec<&mut dyn Focus> {
    /// TODO: Clean up
    fn focus(&mut self, direction: Direction) -> State {
        if let Some((index, _)) = self.iter().enumerate().find(|(_i, e)| e.has_focus()) {
            let state = self
                .get_mut(index)
                .map(|element| element.focus(direction))
                .unwrap_or(State::Ignored);

            if state == State::Returned {
                let is_at_bound = match direction {
                    Direction::Forwards => index == self.len() - 1,
                    Direction::Backwards => index == 0,
                };

                if is_at_bound {
                    State::Returned
                } else {
                    match direction {
                        Direction::Forwards => self.get_mut(index + 1),
                        Direction::Backwards => self.get_mut(index - 1),
                    }
                    .map(|element| element.focus(direction))
                    .unwrap_or(State::Ignored)
                }
            } else {
                state
            }
        } else {
            let beginning = match direction {
                Direction::Forwards => 0,
                Direction::Backwards => self.len() - 1,
            };

            self.get_mut(beginning)
                .map(|element| element.focus(direction))
                .unwrap_or(State::Ignored)
        }
    }

    fn has_focus(&self) -> bool {
        self.iter().any(|t| t.has_focus())
    }
}

impl<T: Focus> Focus for Option<T> {
    fn focus(&mut self, direction: Direction) -> State {
        self.as_mut()
            .map(|t| t.focus(direction))
            .unwrap_or(State::Ignored)
    }

    fn has_focus(&self) -> bool {
        self.as_ref().map(|t| t.has_focus()).unwrap_or(false)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Forwards,
    Backwards,
}
