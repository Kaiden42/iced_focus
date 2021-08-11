//#![doc = include_str!("../README.md")]
//! A proc-macro to derive a focus chain for Iced applications
//! Take a look at the readme for more informations.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,

    // Restriction lints
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::exit,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::let_underscore_must_use,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
)]
#![allow(
    clippy::suboptimal_flops,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::module_name_repetitions
)]

// Thanks to serde: <https://github.com/serde-rs/serde/blob/master/serde/src/lib.rs>
#[cfg(feature = "derive")]
//#[allow(unused_imports)]
extern crate iced_focus_derive;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use iced_focus_derive::*;

/// This trait specifies an element in the applications state that can be addet to the focus chain.
pub trait Focus {
    /// Request a focus for the given direction.    
    fn focus(&mut self, direction: Direction) -> State;
    /// True, if this element has the focus.
    fn has_focus(&self) -> bool;

    // Ugly workaround... see: <https://stackoverflow.com/a/61654763>
    //fn as_dyn(&self) -> &dyn Focus;
    //fn as_dyn_mut(&mut self) -> &mut dyn Focus;
}

/// The state returned by the focus request on a focusable element.
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

impl<T> Focus for Vec<T>
where
    T: Focus,
{
    fn focus(&mut self, direction: Direction) -> State {
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
                .map_or(State::Ignored, |element| element.focus(direction));

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
                .map_or(State::Ignored, |element| element.focus(direction))
        }
    }

    fn has_focus(&self) -> bool {
        self.iter().any(|t| t.has_focus())
    }
}

impl<T: Focus> Focus for Option<T> {
    fn focus(&mut self, direction: Direction) -> State {
        self.as_mut().map_or(State::Ignored, |t| t.focus(direction))
    }

    fn has_focus(&self) -> bool {
        self.as_ref().map_or(false, |t| t.has_focus())
    }
}

/// The direction of the focus request.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    /// Request a forward focus on the focus chain.
    Forwards,
    /// Request a backward focus on the focus chain.
    Backwards,
}
