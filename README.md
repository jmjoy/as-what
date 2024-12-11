# as-what

`as-what` provides a set of custom `as` traits to improve type conversions in Rust, enhancing clarity and readability in expressions.

## Background

The `as` keyword was part of Rust’s early syntax, before the introduction of postfix operators like `.await`. Consider the following example using `as`:

```rust, ignore
let index = y as usize * width.get() as usize + x as usize;
```

In this case, the precedence of the `as` keyword is mixed with other operators, making the expression harder to read. Additionally, `as` does not support chained operations, limiting its flexibility in complex expressions.

## Purpose

The purpose of this library is to provide custom `as` traits that solve these issues, improving the readability of code and allowing more intuitive type conversion expressions. The library helps avoid problems related to operator precedence and enables chaining of operations.

## Features

- Provides various `as` traits for type conversion.
- Improves expression readability by addressing operator precedence issues.
- Supports chaining of operations for more flexible and concise code.

## Installation

```shell
cargo add as-what
```

## Example Usage

Here is a simple example of how to use the library:

```rust, ignore
use as_what::AsUsize;

let index = y.as_usize() * width.get().as_usize() + x.as_usize();
```

## Contributing

If you have any suggestions or find issues, feel free to submit an Issue or Pull Request!

## License

This project is licensed under the MIT License.
