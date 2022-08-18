use std::{borrow::Cow, io::Write};

use crate::custom_reporter::Reporter;
use crate::error::Result;

pub struct JsonReporter<'a, W: Write> {
    started: bool,
    finished: bool,
    indent: usize,
    writer: &'a mut W,
}

impl<'a, W: Write> JsonReporter<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self {
            started: false,
            finished: false,
            indent: 0,
            writer,
        }
    }

    pub fn finish(mut self) -> Result<()> {
        self.writer.write_all("}\n".as_bytes())?;
        self.writer.flush()?;
        self.finished = true;
        Ok(())
    }
}

impl<'a, W: Write> Drop for JsonReporter<'a, W> {
    fn drop(&mut self) {
        if !self.finished {
            let _ = self.writer.write_all("}\n".as_bytes());
        }
    }
}

impl<'a, W: Write> Reporter for JsonReporter<'a, W> {
    fn start_element(&mut self, elem_name: String) -> Result<()> {
        if !self.started {
            self.writer.write_all("{\n".as_bytes())?;
            self.started = true;
            self.indent += 1;
        }
        writeln!(
            self.writer,
            "{}\"{}\": {{",
            "    ".repeat(self.indent),
            escape(&elem_name)
        )?;
        self.indent += 1;
        Ok(())
    }

    fn element_type(&mut self, _: String) -> Result<()> {
        // ignoring
        Ok(())
    }

    fn element_attr(&mut self, attr_name: String, attr_value: String) -> Result<()> {
        writeln!(
            self.writer,
            "{}\"{}\": \"{}\",",
            "    ".repeat(self.indent),
            escape(&attr_name),
            escape(&attr_value)
        )?;
        Ok(())
    }

    fn end_element(&mut self) -> Result<()> {
        debug_assert!(self.indent > 0);
        self.indent -= 1;
        writeln!(self.writer, "{}}},", "    ".repeat(self.indent))?;
        Ok(())
    }
}

fn escape(input: &str) -> Cow<str> {
    for (i, ch) in input.chars().enumerate() {
        if escape_char(ch).is_some() {
            let mut escaped_string = String::with_capacity(input.len());
            escaped_string.push_str(&input[..i]);

            for ch in input[i..].chars() {
                match escape_char(ch) {
                    Some(escaped_char) => escaped_string.push_str(escaped_char),
                    None => escaped_string.push(ch),
                };
            }

            return Cow::Owned(escaped_string);
        }
    }

    Cow::Borrowed(input)
}

fn escape_char(ch: char) -> Option<&'static str> {
    match ch {
        '"' => Some("\\\""),
        _ => None,
    }
}
