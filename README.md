
# Tinter

It prints ANSI colored output. That is all.

## What is it?

This is a *very* minimal implementation for generating ANSI escape sequences in Rust.

* **Basic Colors:** The standard 8 colors and their slightly-more-exciting "bright" variants.
* **A `Color` Enum:**.
* **A `Style` Struct:**. Only foreground color now
* **A `StyledText` Wrapper:** wrapping your text inside glorious colors.

## Why use this (and why you probably shouldn't)

Let's be honest, there are **tons** of other crates out there that do terminal coloring, and they
are generally more robust, feature-rich, and easier to use than this.
Crates like `yansi`, `ansi_term`, and `termcolor`.

### Why to use it

* You have a fear of adding external dependencies, even tiny ones (like me).

### Probably don't use this if

* You need robust, cross-platform terminal coloring.
* You want support for 256 colors, truecolor, or any other fancy features.
* You prefer well-tested, feature-complete libraries.

## Contributing

This code is provided as open source, for anyone who might find it useful or educational.
However, please note that we are not seeking contributions or feature requests. It exists
primarily as a simple example of direct ANSI escape sequence generation.
Feel free to use, modify, or learn from it in your own projects!

## License

MIT

## Copyright

Copyright (c) Peter Bjorklund. All rights reserved.
