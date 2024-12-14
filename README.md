# offset-to-line

Simple and ergonomic lookup of line numbers for offsets in text.

This was written to support development of a [language server](https://microsoft.github.io/language-server-protocol/).

Basic usage is to [parse](Lines::parse) input into [Lines] and then call [offset_line](Lines::offset_line).

## Examples

```rust
use offset_to_line::Lines;
let input = "abcdefg\nhijklmnop\n";
let lines = Lines::parse(input);
assert_eq!(lines.num_lines(), 2, "number of lines is 2");
assert_eq!(lines.offset_line(7).unwrap(), 1, "newline on line 1");
assert_eq!(lines.offset_line(8).unwrap(), 2, "h on line 2");
assert!(lines.offset_line(18).is_err(), "out of bounds");
```

## Similar Crates

* [line-numbers](https://crates.io/crates/line-numbers) has similar functionality with a different design
* [line-span](https://crates.io/crates/line-span) offers more data with a more complex API

