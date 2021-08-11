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
impl<T> Focus for Vec<T>
where
    T: Focus,
{
    fn focus(&mut self, direction: Direction) -> State {
        //let mut vector: Vec<&mut dyn Focus> =
        //    self.iter_mut().map(|t| t as &mut dyn Focus).collect();
        ////vector.as_mut_slice().focus(direction)
        //vector.focus(direction)
        self.as_mut_slice().focus(direction)
    }

    fn has_focus(&self) -> bool {
        self.iter().any(|t| t.has_focus())
    }
}

/// TODO: Change to `as_mut_slice` in the future.
/// See: <https://github.com/rust-lang/rust/issues/76118>
impl<T, const N: usize> Focus for [T; N]
where
    T: Focus,
{
    fn focus(&mut self, direction: Direction) -> State {
        self[..].focus(direction)
    }

    fn has_focus(&self) -> bool {
        self[..].has_focus()
    }
}

/// Ugly workaround.
/// See: <https://users.rust-lang.org/t/why-does-dyn-trait-not-implement-trait/30052>
impl Focus for Box<&mut dyn Focus> {
    fn focus(&mut self, direction: Direction) -> State {
        self.as_mut().focus(direction)
    }

    fn has_focus(&self) -> bool {
        self.as_ref().has_focus()
    }
}

impl<T: Focus> Focus for [T] {
    fn focus(&mut self, direction: Direction) -> State {
        if self.is_empty() {
            return State::Ignored;
        }

        if let Some((index, _)) = self.iter().enumerate().find(|(_i, e)| e.has_focus()) {
            let state = self
                .get_mut(index)
                .map(|element| element.focus(direction))
                .unwrap_or(State::Ignored);

            if state != State::Returned {
                return state;
            }

            let is_at_bound = match direction {
                Direction::Forwards => index == self.len() - 1,
                Direction::Backwards => index == 0,
            };

            if is_at_bound {
                return State::Returned;
            }

            match direction {
                Direction::Forwards => {
                    self[index + 1..]
                        .iter_mut()
                        .find_map(|e| match e.focus(direction) {
                            State::Kept => Some(State::Kept),
                            State::Returned | State::Ignored => None,
                        })
                }
                Direction::Backwards => {
                    self[..index]
                        .iter_mut()
                        .rev()
                        .find_map(|e| match e.focus(direction) {
                            State::Kept => Some(State::Kept),
                            State::Returned | State::Ignored => None,
                        })
                }
            }
            .unwrap_or(State::Ignored)
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
