use std::{borrow::Cow, io::Write};

use crate::custom_reporter::Reporter;
use crate::error::Result;

pub struct HtmlReporter<'a, W: Write> {
    started: bool,
    finished: bool,
    indent: usize,
    writer: &'a mut W,

    prev_element: Option<String>,
}

impl<'a, W: Write> HtmlReporter<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self {
            started: false,
            finished: false,
            indent: 1,
            writer,
            prev_element: None,
        }
    }

    pub fn finish(mut self) -> Result<()> {
        self.writer.write_all(HTML_FOOTER.as_bytes())?;
        self.writer.flush()?;
        self.finished = true;
        Ok(())
    }
}

impl<'a, W: Write> Drop for HtmlReporter<'a, W> {
    fn drop(&mut self) {
        if !self.finished {
            let _ = self.writer.write_all(HTML_FOOTER.as_bytes());
        }
    }
}

impl<'a, W: Write> Reporter for HtmlReporter<'a, W> {
    fn start_element(&mut self, elem_name: String) -> Result<()> {
        if !self.started {
            self.writer.write_all(HTML_HEADER.as_bytes())?;
            self.started = true;
            self.indent += 1;
        }

        self.prev_element = Some(elem_name);

        write!(self.writer, "{}<div", "    ".repeat(self.indent))?;
        self.indent += 1;
        Ok(())
    }

    fn element_type(&mut self, elem_type: String) -> Result<()> {
        writeln!(self.writer, " class=\"{}\">", elem_type)?;

        writeln!(
            self.writer,
            "{}<p class=\"{}\">{}</p>",
            "    ".repeat(self.indent),
            elem_type,
            escape(&(self.prev_element.take().unwrap_or_default()))
        )?;

        Ok(())
    }

    fn element_attr(&mut self, attr_name: String, attr_value: String) -> Result<()> {
        writeln!(
            self.writer,
            "{}<p>{}: {}</p>",
            "    ".repeat(self.indent),
            escape(&attr_name),
            escape(&attr_value)
        )?;
        Ok(())
    }

    fn end_element(&mut self) -> Result<()> {
        debug_assert!(self.indent > 0);
        self.indent -= 1;
        writeln!(self.writer, "{}</div>", "    ".repeat(self.indent))?;
        Ok(())
    }
}

static HTML_HEADER: &str = r#"<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8" />
        <title>HTML report</title>
        <style>
        p.house {font-size: 20px;font-weight: bold;}
        .room {
            background-color: yellow;
            text-indent: 30px;
        }
        .socket {
            background-color: lightgreen;
            text-indent: 60px;
        }
        .thermometer {
            background-color: orange;
            text-indent: 60px;
        }
        </style>
    </head>
    <body>
"#;

static HTML_FOOTER: &str = r#"    </body>
</html>
"#;

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
        '&' => Some("&amp"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&#x27;"),
        _ => None,
    }
}
