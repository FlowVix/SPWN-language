use std::error::Error;
use std::io::{self, Write};

use ahash::AHashMap;
use colored::Colorize;
use line_span::LineSpanExt;
use lyneate::{Report, Theme, ThemeChars};
use supports_color::Stream as CStream;
use supports_unicode::Stream as UStream;

use super::{Diagnostic, RainbowColorGenerator};
use crate::source::SourceMap;

pub trait Emitter: Write {
    fn emit(&mut self, diagnostic: &Diagnostic) -> Result<(), Box<dyn Error>>;

    fn support_color(&self) -> bool {
        std::env::var("NO_COLOR").map_or(false, |v| !v.is_empty())
            || supports_color::on(CStream::Stderr).is_some()
    }
    fn supports_unicode(&self) -> bool {
        supports_unicode::on(UStream::Stderr)
    }
}

/// Outputs to stderr
pub struct StandardEmitter {
    pub(crate) src_map: SourceMap,
}
impl StandardEmitter {
    pub fn new(src_map: SourceMap) -> Self {
        Self { src_map }
    }
}

impl Write for StandardEmitter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::stderr().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stderr().flush()
    }
}
impl Emitter for StandardEmitter {
    fn emit(&mut self, diagnostic: &Diagnostic) -> Result<(), Box<dyn Error>> {
        // `colored` already follows the NO_COLOR behavior so we dont have to check it here
        writeln!(
            self,
            "\n{}: {}",
            diagnostic.title.color(diagnostic.level).bold(),
            diagnostic.message
        )?;

        let mut src_map = AHashMap::default();

        for (msg, span) in &diagnostic.labels {
            let source_file = self
                .src_map
                .get_file_by_id(span.source_id)
                // TODO: remove expects and use errors
                .expect("BUG: unknown source id in diagnostic label");
            let source = source_file
                .src
                .as_ref()
                .expect("BUG: no source with source file");

            src_map
                .entry(source_file.name.clone())
                .or_insert_with(|| (source.clone(), vec![]))
                .1
                .push((*span, msg));
        }

        let theme_chars = if self.supports_unicode() {
            ThemeChars {
                side_vertical_dotted: '·',
                ..Default::default()
            }
        } else {
            ThemeChars {
                side_vertical_dotted: '·',
                ..ThemeChars::ascii()
            }
        };

        let mut colors = RainbowColorGenerator::new(345.0, 0.75, 1.0, 45.0);
        let theme = Theme {
            chars: theme_chars,
            ..Default::default()
        };

        for (src, (code, labels)) in src_map {
            writeln!(
                self,
                "{}{}{}\n",
                "[".dimmed(),
                src.truecolor(123, 184, 255),
                "]".dimmed(),
            )?;
            writeln!(
                self,
                "{}",
                Report::new_byte_spanned(
                    &code,
                    labels.into_iter().map(|(span, msg)| {
                        (
                            span.into(),
                            msg.truecolor(150, 150, 150).to_string(),
                            colors.next(),
                        )
                    }),
                )
                .with_theme(theme)
                .display_str()
            )?;
        }

        self.flush()?;
        Ok(())
    }
}
