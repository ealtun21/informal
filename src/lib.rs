//! Simple crate for parsing user input.
//!
//! # Examples
//!
//! Rust type inference is used to know what to return.
//!
//! ```no_run
//! let username: String = informal::prompt("Please enter your name: ").get();
//! ```
//!
//! [`FromStr`] is used to parse the input, so you can read any type that
//! implements [`FromStr`].
//!
//! ```no_run
//! let age: u32 = informal::prompt("Please enter your age: ").get();
//! ```
//!
//! [`.matches()`] can be used to validate the input data.
//!
//! ```no_run
//! let age: u32 = informal::prompt("Please enter your age again: ")
//!     .matches(|x| *x < 120)
//!     .get();
//! ```
//!
//! [`.type_error_message()`] can be used to specify an error message when the string fails to be converted into the wanted type.
//!
//! ```no_run
//! let age: u32 = informal::prompt("Please enter your age: ")
//!     .type_error_message("Error: What kind of age is that?!")
//!     .get();
//! ```
//! 
//! [`.validator_error_message()`] can be used to specify an error message when your matches condition does not hold.
//!
//! ```no_run
//! let age: u32 = informal::prompt("Please enter your age: ")
//!     .matches(|x| *x < 120)
//!     .validator_error_message("Error: You can't be that old.... can you?")
//!     .get();
//! ```
//!
//! A convenience function [`confirm`] is provided for getting a yes or no
//! answer.
//!
//! ```no_run
//! if informal::confirm("Are you sure you want to continue?") {
//!     // continue
//! } else {
//!     panic!("Aborted!");
//! }
//! ```
//! 
//! //! A convenience function [`confirm_with_message`] is provided for getting a yes or no
//! answer with an error message.
//!
//! ```no_run
//! if informal::confirm_with_message("Are you sure you want to continue?", "Please answer with 'yes' or 'no'") {
//!     // continue
//! } else {
//!     panic!("Aborted!");
//! }
//! ```
//!
//! [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
//! [`.matches()`]: struct.Input.html#method.matches
//! [`confirm`]: fn.confirm.html

use std::fmt::{self, Debug, Display};
use std::io::{self, Write};
use std::str::FromStr;

/////////////////////////////////////////////////////////////////////////
// Definitions
/////////////////////////////////////////////////////////////////////////

/// A validator for user input.
struct Validator<T> {
    raw: Box<dyn Fn(&T) -> bool + 'static>,
}

/// An input builder.
pub struct Input<T> {
    prompt: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
    default: Option<T>,
    validator: Option<Validator<T>>,
    type_message: Option<String>,
    validator_message: Option<String>,
}

/////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////

impl<T> Validator<T> {
    /// Construct a new `Validator`.
    fn new<F>(raw: F) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        Self { raw: Box::new(raw) }
    }

    /// Run the validator on the given input.
    fn run(&self, input: &T) -> bool {
        (self.raw)(input)
    }
}

impl<T: Debug> Debug for Input<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Input")
            .field("prefix", &self.prefix)
            .field("prompt", &self.prompt)
            .field("suffix", &self.suffix)
            .field("default", &self.default)
            .finish_non_exhaustive()
    }
}

impl<T> Default for Input<T> {
    /// Construct a new empty `Input`.
    ///
    /// Identical to [`Input::new()`](struct.Input.html#method.new).
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Input<T> {
    /// Construct a new empty `Input`.
    ///
    /// Identical to [`Input::default()`](struct.Input.html#impl-Default).
    pub fn new() -> Self {
        Self {
            prefix: None,
            prompt: None,
            suffix: None,
            default: None,
            validator: None,
            type_message: Some(String::from("Error: invalid input")),
            validator_message: Some(String::from("Error: does not pass validation")),
        }
    }

    /// Set the prompt to display before waiting for user input.
    pub fn prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    /// Set the prompt prefix.
    pub fn prefix<S: Into<String>>(mut self, prefix: S) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set the prompt suffix.
    pub fn suffix<S: Into<String>>(mut self, suffix: S) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Set the error message that is printed when the type convertion fails.
    pub fn type_error_message<S: Into<String>>(mut self, message: S) -> Self {
        self.type_message = Some(message.into());
        self
    }

    /// Set the error message that is printed when the validator condition is not met.
    pub fn validator_error_message<S: Into<String>>(mut self, message: S) -> Self {
        self.validator_message = Some(message.into());
        self
    }

    /// Set the default value.
    ///
    /// If set, this will be returned in the event the user enters an empty
    /// input.
    pub fn default(mut self, default: T) -> Self {
        self.default = Some(default);
        self
    }

    /// Check input values.
    ///
    /// If set, this function will be called on the parsed user input and only
    /// if it passes will we return the value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use informal::Input;
    /// let num: u32 = Input::new().matches(|x| *x != 10).get();
    /// ```
    pub fn matches<F>(mut self, matches: F) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.validator = Some(Validator::new(matches));
        self
    }
}

fn read_line(prompt: &Option<String>) -> io::Result<String> {
    if let Some(prompt) = prompt {
        let mut stdout = io::stdout();
        stdout.write_all(prompt.as_bytes())?;
        stdout.flush()?;
    }
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

impl<T> Input<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    fn try_get_with<F>(self, read_line: F) -> io::Result<T>
    where
        F: Fn(&Option<String>) -> io::Result<String>,
    {
        let Self {
            prompt,
            prefix,
            suffix,
            default,
            validator,
            type_message: error_message,
            validator_message,
        } = self;

        let prompt = prompt.map(move |prompt| {
            let mut p = String::new();
            if let Some(prefix) = prefix {
                p.push_str(&prefix);
            }
            p.push_str(&prompt);
            if let Some(suffix) = suffix {
                p.push_str(&suffix);
            }
            p
        });

        Ok(loop {
            match read_line(&prompt)?.trim() {
                "" => {
                    if let Some(default) = default {
                        break default;
                    } else {
                        continue;
                    }
                }
                raw => match raw.parse() {
                    Ok(result) => {
                        if let Some(validator) = &validator {
                            if !validator.run(&result) {
                                println!(
                                    "{}",
                                    validator_message.as_ref().unwrap_or(&"".to_string())
                                );
                                continue;
                            }
                        }
                        break result;
                    }
                    Err(err) => {
                        println!(
                            "{}",
                            error_message
                                .as_ref()
                                .unwrap_or(&format!("Error: {}", err).to_string())
                        );
                        continue;
                    }
                },
            }
        })
    }

    #[inline]
    fn try_get(self) -> io::Result<T> {
        self.try_get_with(read_line)
    }

    /// Consumes the `Input` and reads the input from the user.
    ///
    /// This function uses [`FromStr`] to parse the input data.
    ///
    /// ```no_run
    /// # use informal::Input;
    /// let num: u32 = Input::new().prompt("Enter a number: ").get();
    /// ```
    ///
    /// [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
    pub fn get(self) -> T {
        self.try_get().unwrap()
    }

    /// Consumes the `Input` and applies the given function to it.
    ///
    /// This function uses [`FromStr`] to parse the input data. The result is
    /// then fed to the given closure.
    ///
    /// ```no_run
    /// # use informal::Input;
    /// let value = Input::new().map(|s: String| &s.to_lowercase() == "yes");
    /// ```
    ///
    /// [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
    pub fn map<F, U>(self, map: F) -> U
    where
        F: Fn(T) -> U,
    {
        map(self.get())
    }
}

/////////////////////////////////////////////////////////////////////////
// Shortcut functions
/////////////////////////////////////////////////////////////////////////

/// Returns a new empty `Input`.
///
/// # Examples
///
/// Read in something without any prompt.
///
/// ```no_run
/// # use informal::input;
/// let data: String = input().get();
/// ```
pub fn input<T>() -> Input<T> {
    Input::new()
}

/// Returns an `Input` that prompts the user for input.
///
/// # Examples
///
/// Read in a simple string:
///
/// ```no_run
/// # use informal::prompt;
/// let username: String = prompt("Please enter your name: ").get();
/// ```
///
/// Types that implement [`FromStr`] will be automatically parsed.
///
/// ```no_run
/// # use informal::prompt;
/// let years = prompt("How many years have you been coding Rust: ")
///     .default(0)
///     .get();
/// ```
///
/// [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
pub fn prompt<S, T>(text: S) -> Input<T>
where
    S: Into<String>,
{
    Input::new().prompt(text)
}

/// Prompts the user for confirmation (yes/no).
///
/// # Examples
///
/// ```no_run
/// # use informal::confirm;
/// if confirm("Are you sure you want to continue?") {
///     // continue
/// } else {
///     panic!("Aborted!");
/// }
/// ```
pub fn confirm<S: Into<String>>(text: S) -> bool {
    prompt(text)
        .suffix(" [y/N] ")
        .default("n".to_string())
        .matches(|s| matches!(&*s.trim().to_lowercase(), "n" | "no" | "y" | "yes"))
        .map(|s| matches!(&*s.to_lowercase(), "y" | "yes"))
}

/// Prompts the user for confirmation (yes/no) with an error message.
///
/// # Examples
///
/// ```no_run
/// # use informal::confirm;
/// if confirm("Are you sure you want to continue?") {
///     // continue
/// } else {
///     panic!("Aborted!");
/// }
/// ```
pub fn confirm_with_message<S: Into<String>>(text: S, error_meesage: S) -> bool {
    prompt(text)
        .suffix(" [y/N] ")
        .default("n".to_string())
        .validator_error_message(error_meesage)
        .matches(|s| matches!(&*s.trim().to_lowercase(), "n" | "no" | "y" | "yes"))
        .map(|s| matches!(&*s.to_lowercase(), "y" | "yes"))
}
