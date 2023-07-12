//! Beautiful, minimal, opinionated CLI prompts inspired by the
//! [@clack/prompts](https://www.npmjs.com/package/@clack/prompts) `npm` package.
//!
//! "Effortlessly build beautiful command-line apps" (C)
//! [original @clack](https://www.npmjs.com/package/@clack/prompts).
//!
//! 💎 Fancy minimal UI.<br>
//! ✅ Simple API<br>
//! 🧱 Comes with [`input`](fn@input), [`password`](fn@password),
//!    [`confirm`](fn@confirm), [`select`](fn@select),
//!    [`multiselect`](fn@multiselect), and [`spinner`](fn@spinner) prompts.<br>
//! 🧱 [`log`] submodule allows printing styled non-interactive messages.<br>
//!
//! <img src="./cliclack-demo.gif" width="40%">
//!
//! # Usage
//!
//! API is similar to the original Clack API besides of a few exceptions.
//!
//! ## Setup
//!
//! The [`intro`] and [`outro`]/[`outro_cancel`] functions will
//! print a message to begin and end a prompt session respectively.
//!
//! ```
//! use cliclack::{intro, outro};
//!
//! intro("create-my-app")?;
//! // Do stuff
//! outro("You're all set!")?;
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## Cancellation
//!
//! `Esc` cancels the prompt sequence with a nice message.
//! `Ctrl+C` interrupts the session abruptly, it's handled inside of the
//! `Term` crate and cannot be easily caught and rendered fancy.
//!
//! # Components
//!
//! All prompts can be constructed either directly, e.g. with [`Input::new`],
//! or with the convenience function, e.g. [`input()`].
//!
//! ## Input
//!
//! The input prompt accepts a single line of text trying to parse it into
//! a target type.
//!
//! ```
//! use cliclack::input;
//!
//! # fn test() -> std::io::Result<()> {
//! let number: String = input("What is the meaning of life?")
//!     .placeholder("Not sure")
//!     .validate(|input: &String| {
//!         if input.is_empty() {
//!             Err("Value is required!")
//!         } else {
//!             Ok(())
//!         }
//!     })
//!     .interact()?;
//! # Ok(())
//! # }
//! # test().ok(); // Ignoring I/O runtime errors.
//! ```
//!
//! ## Password
//!
//! The password prompt is similar to the input prompt, but it doesn't echo the
//! actual characters.
//!
//! ```
//! # fn test() -> std::io::Result<()> {
//! use cliclack::password;
//!
//! let password = password("Provide a password")
//!     .mask('▪')
//!     .interact()?;
//! # Ok(())
//! # }
//! # test().ok(); // Ignoring I/O runtime errors.
//! ```
//!
//! ## Confirm
//!
//! The confirm prompt asks for a yes/no answer. It returns a boolean (`true`/`false`).
//!
//! '`Y`' and '`N`' keys are accepted as an immediate answer.
//!
//! ```
//! # fn test() -> std::io::Result<()> {
//! use cliclack::confirm;
//!
//! let should_continue = confirm("Do you want to continue?").interact()?;
//! # Ok(())
//! # }
//! # test().ok(); // Ignoring I/O runtime errors.
//! ```
//!
//! ## Select
//!
//! The select prompt asks to choose one of the options from the list.
//!
//! ```
//! # fn test() -> std::io::Result<()> {
//! use cliclack::select;
//!
//! let selected = select("Pick a project type")
//!     .item("ts", "TypeScript", "")
//!     .item("js", "JavaScript", "")
//!     .item("coffee", "CoffeeScript", "oh no")
//!     .interact()?;
//! # Ok(())
//! # }
//! # test().ok(); // Ignoring I/O runtime errors.
//! ```
//!
//! ## Multi-Select
//!
//! The multi-select prompt asks to choose one or more options from the list.
//! The result is a vector of selected items.
//!
//! ```
//! # fn test() -> std::io::Result<()> {
//! use cliclack::multiselect;
//!
//! let additional_tools = multiselect("Select additional tools.")
//!     .item("eslint", "ESLint", "recommended")
//!     .item("prettier", "Prettier", "")
//!     .item("gh-action", "GitHub Actions", "")
//!     .interact()?;
//! # Ok(())
//! # }
//! # test().ok(); // Ignoring I/O runtime errors.
//! ```
//!
//! ## Spinner
//!
//! ```
//! # fn test() -> std::io::Result<()> {
//! use cliclack::spinner;
//!
//! let mut spinner = spinner();
//! spinner.start("Installing...");
//! // Do installation.
//! spinner.stop("Installation complete");
//! # Ok(())
//! # }
//! # test().ok(); // Ignoring I/O runtime errors.
//! ```

mod confirm;
mod input;
mod multiselect;
mod password;
mod prompt;
mod select;
mod spinner;
mod theme;
mod validate;

use console::Term;
use std::fmt::Display;
use std::io;

use theme::{ClackTheme, Theme};

pub use confirm::Confirm;
pub use input::Input;
pub use multiselect::MultiSelect;
pub use password::Password;
pub use select::Select;
pub use spinner::Spinner;
pub use validate::Validate;

fn term_write(line: String) -> io::Result<()> {
    Term::stderr().write_str(&line)
}

/// Clears the terminal.
pub fn clear_screen() -> io::Result<()> {
    Term::stdout().clear_screen()?;
    Term::stderr().clear_screen()
}

/// Prints a header of the prompt sequence.
pub fn intro(title: impl Display) -> io::Result<()> {
    term_write(ClackTheme.format_intro(&title.to_string()))
}

/// Prints a footer of the prompt sequence.
pub fn outro(message: impl Display) -> io::Result<()> {
    term_write(ClackTheme.format_outro(&message.to_string()))
}

/// Prints a footer of the prompt sequence with a failure style.
pub fn outro_cancel(message: impl Display) -> io::Result<()> {
    term_write(ClackTheme.format_cancel(&message.to_string()))
}

/// Constructs a new [`Input`] prompt.
///
/// See [`Input`] for chainable methods.
pub fn input(prompt: impl Display) -> Input {
    Input::new(prompt)
}

/// Constructs a new [`Password`] prompt.
///
/// See [`Password`] for chainable methods.
pub fn password(prompt: impl Display) -> Password {
    Password::new(prompt)
}

/// Constructs a new [`Select`] prompt.
///
/// See [`Select`] for chainable methods.
pub fn select<T: Default + Clone + Eq>(prompt: impl Display) -> Select<T> {
    Select::new(prompt)
}

/// Constructs a new [`MultiSelect`] prompt.
///
/// See [`MultiSelect`] for chainable methods.
pub fn multiselect<T: Default + Clone + Eq>(prompt: impl Display) -> MultiSelect<T> {
    MultiSelect::new(prompt)
}

/// Constructs a new [`Confirm`] prompt.
///
/// See [`Confirm`] for chainable methods.
pub fn confirm(prompt: impl Display) -> Confirm {
    Confirm::new(prompt)
}

/// Constructs a new [`Spinner`] prompt.
///
/// See [`Spinner`] for chainable methods.
pub fn spinner() -> Spinner {
    Spinner::default()
}

/// Prints a note message.
pub fn note(prompt: impl Display, message: impl Display) -> io::Result<()> {
    term_write(ClackTheme.format_note(&prompt.to_string(), &message.to_string()))
}

/// Non-interactive information messages of different styles.
pub mod log {
    use super::*;

    fn log(text: impl Display, symbol: impl Display) -> io::Result<()> {
        term_write(ClackTheme.format_log(&text.to_string(), &symbol.to_string()))
    }

    /// Prints a remark message.
    pub fn remark(text: impl Display) -> io::Result<()> {
        log(text, ClackTheme.remark_symbol())
    }

    /// Prints an info message.
    pub fn info(text: impl Display) -> io::Result<()> {
        log(text, ClackTheme.info_symbol())
    }

    /// Prints a warning message.
    pub fn warning(message: impl Display) -> io::Result<()> {
        log(message, ClackTheme.warning_symbol())
    }

    /// Prints an error message.
    pub fn error(message: impl Display) -> io::Result<()> {
        log(message, ClackTheme.error_symbol())
    }

    /// Prints a success message.
    pub fn success(message: impl Display) -> io::Result<()> {
        log(message, ClackTheme.active_symbol())
    }

    /// Prints a submitted step message.
    pub fn step(message: impl Display) -> io::Result<()> {
        log(message, ClackTheme.submit_symbol())
    }
}
