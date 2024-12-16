# line-position

Simple and ergonomic lookup of line numbers for offsets in text.

This was written to support development of a
[language server](https://microsoft.github.io/language-server-protocol/).

## Examples

Basic usage is to parse input into `Lines` and then call `position()`.
Then you can access the one-indexed line number and zero-indexed offset via methods.

```rust
use line_position::{Lines, LinePosition};

let input: &str = "abcdefg\nhijklmnop\n";
let lines: Lines = Lines::parse(input);
let line_position: LinePosition = lines.position(5).unwrap();
assert_eq!(lines.num_lines(), 2, "number of lines is 2");
assert_eq!(line_position.line(), 1, "f on line 1");
assert_eq!(line_position.offset(), 5, "f at line offset 5");
assert!(lines.position(18).is_err(), "out of bounds");
```

## Similar Crates

* [line-numbers](https://crates.io/crates/line-numbers) has similar functionality with a
  different design
* [line-span](https://crates.io/crates/line-span) offers more data with a more complex API

