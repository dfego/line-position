//! Simple and ergonomic lookup of line numbers for offsets in text.
//!
//! This was written to support development of a [language server](https://microsoft.github.io/language-server-protocol/).
//!
//! Basic usage is to [parse](Lines::parse) input into [Lines] and then call [offset_line](Lines::offset_line).
//!
//! # Examples
//!
//! ```
//! use offset_to_line::Lines;
//! let input = "abcdefg\nhijklmnop\n";
//! let lines = Lines::parse(input);
//! assert_eq!(lines.num_lines(), 2, "number of lines is 2");
//! assert_eq!(lines.offset_line(7).unwrap(), 1, "newline on line 1");
//! assert_eq!(lines.offset_line(8).unwrap(), 2, "h on line 2");
//! assert!(lines.offset_line(18).is_err(), "out of bounds");
//! ```
//!
//! # Similar Crates
//!
//! * [line-numbers](https://crates.io/crates/line-numbers) has similar functionality with a different design
//! * [line-span](https://crates.io/crates/line-span) offers more data with a more complex API
//!
#![warn(missing_docs)]

/// Error type for this library.
/// At present, only one error exists, but enum is to future-proof.
#[derive(Debug)]
pub enum LinesError {
    /// Offset passed to [offset_line](Lines::offset_line) was beyond the bounds of the string parsed by [parse](Lines::parse).
    OffsetOutOfBounds,
}

type LineNumber = u32;
type LinesResult = Result<LineNumber, LinesError>;

#[derive(Debug)]
struct Line {
    start: usize,
    end: usize,
}

/// Parser for string data that exposes methods for querying offsets.
///
/// Simply:
/// 1. Parse a string with [parse](Lines::parse)
/// 2. Call [offset_line](Lines::offset_line) with an offset to get a line number.
///
/// See the [main page](crate) for a full example.
#[derive(Debug)]
pub struct Lines {
    lines: Vec<Line>,
}

impl Lines {
    /// Parse the given input string, storing the line data in the returned value.
    ///
    /// The parser assumes line endings are consistent, i.e. all `\n` or all `\r\n`.
    /// As a consequence, if the input contains an `\r\n`, that is the delimiter used.
    pub fn parse(input: &str) -> Self {
        let mut lines = Vec::new();
        let line_ending = match input.contains("\r\n") {
            true => "\r\n",
            false => "\n",
        };

        let mut start: usize = 0;
        for input_line in input.split_inclusive(line_ending) {
            let end = start + input_line.len();
            let line = Line { start, end };
            start = end;
            lines.push(line);
        }

        Lines { lines }
    }

    /// Lookup the line number for a given character offset within the parsed string.
    ///
    /// Returns a [Result] containing either a line number on success, or [LinesError] on failure.
    ///
    /// The only possible error here is [OffsetOutOfBounds](LinesError::OffsetOutOfBounds), which occurs if the offset is beyond the length of the input.
    pub fn offset_line(&self, offset: usize) -> LinesResult {
        let mut line_number = 1u32;
        for line in &self.lines {
            if offset >= line.start && offset < line.end {
                return Ok(line_number);
            }
            line_number += 1
        }
        Err(LinesError::OffsetOutOfBounds)
    }

    /// Return the number of lines parsed.
    ///
    /// Note that if the text ends with the end-of-line delimiter, it does *not* count new line after that.
    /// See tests for an example of this.
    pub fn num_lines(&self) -> usize {
        self.lines.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_line_no_eol() -> Result<(), LinesError> {
        let input = "abcdefg";
        let lines = Lines::parse(input);
        assert_eq!(lines.num_lines(), 1, "number of lines is 1");
        assert_eq!(lines.offset_line(0)?, 1, "a on line 1");
        assert_eq!(lines.offset_line(6)?, 1, "g on line 1");
        assert!(lines.offset_line(7).is_err(), "out of bounds");

        Ok(())
    }

    #[test]
    fn one_line_lf() -> Result<(), LinesError> {
        let input = "abcdefg\n";
        let lines = Lines::parse(input);

        assert_eq!(lines.num_lines(), 1, "number of lines is 1");
        assert_eq!(lines.offset_line(0)?, 1, "a on line 1");
        assert_eq!(lines.offset_line(7)?, 1, "newline on line 1");
        assert!(lines.offset_line(8).is_err(), "out of bounds");

        Ok(())
    }

    #[test]
    fn one_line_crlf() -> Result<(), LinesError> {
        let input = "abcdefg\r\n";
        let lines = Lines::parse(input);

        assert_eq!(lines.num_lines(), 1, "number of lines is 1");
        assert_eq!(lines.offset_line(0)?, 1, "a on line 1");
        assert_eq!(lines.offset_line(8)?, 1, "newline on line 1");
        assert!(lines.offset_line(9).is_err(), "out of bounds");

        Ok(())
    }

    #[test]
    fn two_lines_lf() -> Result<(), LinesError> {
        let input = "abcdefg\nhijklmnop\n";
        let lines = Lines::parse(input);

        assert_eq!(lines.num_lines(), 2, "number of lines is 2");
        assert_eq!(lines.offset_line(7)?, 1, "newline on line 1");
        assert_eq!(lines.offset_line(8)?, 2, "h on line 2");

        Ok(())
    }

    #[test]
    fn two_lines_crlf() -> Result<(), LinesError> {
        let input = "abcdefg\r\nhijklmnop\r\n";
        let lines = Lines::parse(input);

        assert_eq!(lines.num_lines(), 2, "number of lines is 2");
        assert_eq!(lines.offset_line(8)?, 1, "newline on line 1");
        assert_eq!(lines.offset_line(9)?, 2, "h on line 2");

        Ok(())
    }

    #[test]
    fn mixed_eol() -> Result<(), LinesError> {
        let input = "abcdefg\r\nhijklmnop\nqrstuv";
        let lines = Lines::parse(input);

        assert_eq!(lines.num_lines(), 2, "number of lines is 2");
        assert_eq!(lines.offset_line(8)?, 1, "first newline on line 1");
        assert_eq!(lines.offset_line(9)?, 2, "h on line 2");
        assert_eq!(lines.offset_line(24)?, 2, "v on line 2");
        assert!(lines.offset_line(25).is_err(), "out of bounds");

        Ok(())
    }
}
