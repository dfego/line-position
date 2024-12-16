//! Simple and ergonomic lookup of line numbers for offsets in text.
//!
//! This was written to support development of a
//! [language server](https://microsoft.github.io/language-server-protocol/).
//!
//! # Examples
//!
//! Basic usage is to [parse](Lines::parse) input into [Lines] and then call [position](Lines::position).
//! Then you can access the one-indexed line number and zero-indexed offset within the line via
//! [line](LinePosition::line) and [offset](LinePosition::offset).
//!
//! ```
//! use line_position::{Lines, LinePosition};
//!
//! let input: &str = "abcdefg\nhijklmnop\n";
//! let lines: Lines = Lines::parse(input);
//! let line_position: LinePosition = lines.position(5).unwrap();
//! assert_eq!(lines.num_lines(), 2, "number of lines is 2");
//! assert_eq!(line_position.line(), 1, "f on line 1");
//! assert_eq!(line_position.offset(), 5, "f at line offset 5");
//! assert!(lines.position(18).is_err(), "out of bounds");
//! ```
//!
//! # Similar Crates
//!
//! * [line-numbers](https://crates.io/crates/line-numbers) has similar functionality with a
//!   different design
//! * [line-span](https://crates.io/crates/line-span) offers more data with a more complex API
//!
#![warn(missing_docs)]

/// Error type for this crate.
#[derive(Debug)]
pub enum LinesError {
    /// The offset passed to [position][Lines::position] was beyond the length of the input.
    OffsetOutOfBounds,
}

type LinesResult = Result<LinePosition, LinesError>;

/// Position within the file.
#[derive(Debug)]
pub struct LinePosition {
    line: usize,
    offset: usize,
}

impl LinePosition {
    /// Line number of position, starting with 1.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Offset within the line of the given position, starting with 0.
    pub fn offset(&self) -> usize {
        self.offset
    }
}

#[derive(Debug)]
struct Line {
    start: usize,
    end: usize,
}

/// Parser for string data that exposes methods for querying offsets.
///
/// Simply:
/// 1. Parse a string with [parse](Lines::parse)
/// 2. Call [position](Lines::position) with an offset to get a [LinePosition].
/// 3. Use [line][LinePosition::line] to access the line number and [offset][LinePosition::offset] to access the line offset.
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
    pub fn position(&self, input_offset: usize) -> LinesResult {
        let mut line_number = 1usize;
        for line in &self.lines {
            if input_offset >= line.start && input_offset < line.end {
                return Ok(LinePosition {
                    line: line_number,
                    offset: input_offset - line.start,
                });
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
        assert_eq!(lines.position(0)?.line, 1, "a on line 1");
        assert_eq!(lines.position(6)?.line, 1, "g on line 1");
        assert!(lines.position(7).is_err(), "out of bounds");

        Ok(())
    }

    #[test]
    fn one_line_lf() -> Result<(), LinesError> {
        let input = "abcdefg\n";
        let lines = Lines::parse(input);

        assert_eq!(lines.num_lines(), 1, "number of lines is 1");
        assert_eq!(lines.position(0)?.line, 1, "a on line 1");
        assert_eq!(lines.position(7)?.line, 1, "newline on line 1");
        assert!(lines.position(8).is_err(), "out of bounds");

        Ok(())
    }

    #[test]
    fn one_line_crlf() -> Result<(), LinesError> {
        let input = "abcdefg\r\n";
        let lines = Lines::parse(input);

        assert_eq!(lines.num_lines(), 1, "number of lines is 1");
        assert_eq!(lines.position(0)?.line, 1, "a on line 1");
        assert_eq!(lines.position(8)?.line, 1, "newline on line 1");
        assert!(lines.position(9).is_err(), "out of bounds");

        Ok(())
    }

    #[test]
    fn two_lines_lf() -> Result<(), LinesError> {
        let input = "abcdefg\nhijklmnop\n";
        let lines = Lines::parse(input);

        assert_eq!(lines.num_lines(), 2, "number of lines is 2");
        assert_eq!(lines.position(7)?.line(), 1, "newline on line 1");
        assert_eq!(lines.position(8)?.line(), 2, "h on line 2");
        assert_eq!(lines.position(8)?.offset(), 0, "h at offset 0");

        Ok(())
    }

    #[test]
    fn two_lines_crlf() -> Result<(), LinesError> {
        let input = "abcdefg\r\nhijklmnop\r\n";
        let lines = Lines::parse(input);

        assert_eq!(lines.num_lines(), 2, "number of lines is 2");
        assert_eq!(lines.position(8)?.line(), 1, "newline on line 1");
        assert_eq!(lines.position(9)?.line(), 2, "h on line 2");
        assert_eq!(lines.position(9)?.offset(), 0, "h at offset 0");

        Ok(())
    }

    #[test]
    fn mixed_eol() -> Result<(), LinesError> {
        let input = "abcdefg\r\nhijklmnop\nqrstuv";
        let lines = Lines::parse(input);

        assert_eq!(lines.num_lines(), 2, "number of lines is 2");
        assert_eq!(lines.position(8)?.line(), 1, "first newline on line 1");
        assert_eq!(lines.position(9)?.line(), 2, "h on line 2");
        assert_eq!(lines.position(24)?.line(), 2, "v on line 2");
        assert!(lines.position(25).is_err(), "out of bounds");

        Ok(())
    }
}
