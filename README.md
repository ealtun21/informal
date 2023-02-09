# Informal

Simple crate for parsing user input.

## Getting started

Add the following dependency to your `Cargo.toml`.

```toml
[dependencies]
informal = "0.3"
```

## Usage

Rust type inference is used to know what to return.

## Examples

Rust type inference is used to know what to return.

```rust
let username: String = informal::prompt("Please enter your name: ").get();
```

[`FromStr`] is used to parse the input, so you can read any type that
implements [`FromStr`].

```rust
let age: u32 = informal::prompt("Please enter your age: ").get();
```

[`.matches()`] can be used to validate the input data.

```rust
let age: u32 = informal::prompt("Please enter your age again: ")
    .matches(|x| *x < 120)
    .get();
```

[`.type_error_message()`] can be used to specify an error message when the string fails to be converted into the wanted type.

```rust
let age: u32 = informal::prompt("Please enter your age: ")
    .type_error_message("Error: What kind of age is that?!")
    .get();
```

[`.validator_error_message()`] can be used to specify an error message when your matches condition does not hold.

```rust
let age: u32 = informal::prompt("Please enter your age: ")
    .matches(|x| *x < 120)
    .validator_error_message("Error: You can't be that old.... can you?")
    .get();
```

A convenience function [`confirm`] is provided for getting a yes or no
answer.

```rust
if informal::confirm("Are you sure you want to continue?") {
    // continue
} else {
    panic!("Aborted!");
}
```

 A convenience function [`confirm_with_message`] is provided for getting a yes or no
answer with an error message.

```rust
if informal::confirm_with_message("Are you sure you want to continue?", "Please answer with 'yes' or 'no'") {
    // continue
} else {
    panic!("Aborted!");
}
```

[`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[`.matches()`]: struct.Input.html#method.matches
[`confirm`]: fn.confirm.html

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
