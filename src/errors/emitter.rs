use std::io::{self, Write};

use ahash::AHashMap;
use colored::Colorize;
use lyneate::{Report, Theme, ThemeChars};
use supports_color::Stream as CStream;
use supports_unicode::Stream as UStream;

use super::Diagnostic;
use crate::error::RainbowColorGenerator;

pub trait Emitter: Write {
    fn emit(&mut self, diagnostic: &Diagnostic) -> io::Result<()>;

    fn support_color(&self) -> bool {
        std::env::var("NO_COLOR").map_or(false, |v| !v.is_empty())
            || supports_color::on(CStream::Stderr).is_some()
    }
    fn supports_unicode(&self) -> bool {
        supports_unicode::on(UStream::Stderr)
    }
}

/// Outputs to stderr
pub struct StandardEmitter;

impl Write for StandardEmitter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        std::io::stderr().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stderr().flush()
    }
}
impl Emitter for StandardEmitter {
    fn emit(&mut self, diagnostic: &Diagnostic) -> io::Result<()> {
        // `colored`` already follows the NO_COLOR behavior so we dont have to check it here
        writeln!(
            self,
            "\n{}: {}",
            diagnostic.title.color(diagnostic.level).bold(),
            diagnostic.message
        )?;

        // TODO: can this source map come from somewhere else?
        let mut src_map = AHashMap::default();

        for (msg, area) in &diagnostic.labels {
            src_map
                .entry(area.src.name())
                .or_insert_with(|| (area.src.read().unwrap(), vec![]))
                .1
                .push((area.span, msg));
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
            write!(
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

            writeln!(self)?;
        }

        self.flush()
    }
}
