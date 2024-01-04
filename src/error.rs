use std::error::Error;
use std::fmt::Display;

use ahash::AHashMap;
use colored::Colorize;
use lyneate::{Report, Theme, ThemeChars};

use crate::source::CodeArea;
use crate::util::hsv_to_rgb;

#[derive(Debug, Clone, Copy)]
pub struct RainbowColorGenerator {
    h: f64,
    s: f64,
    v: f64,
    hue_shift: f64,
}

impl RainbowColorGenerator {
    pub fn new(h: f64, s: f64, v: f64, hue_shift: f64) -> Self {
        Self { h, s, v, hue_shift }
    }

    pub fn next(&mut self) -> (u8, u8, u8) {
        let h0 = self.h / 360.0;

        self.h = (self.h + self.hue_shift).rem_euclid(360.0);

        hsv_to_rgb(h0, self.s, self.v)
    }
}

#[derive(Debug)]
pub struct ErrorReport {
    pub title: &'static str,
    pub msg: &'static str,

    pub labels: Vec<(CodeArea, String)>,
}

impl Error for ErrorReport {}

impl Display for ErrorReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\n{}: {}",
            self.title.truecolor(255, 72, 72).bold(),
            self.msg
        )?;

        let mut src_map = AHashMap::default();

        for (area, msg) in &self.labels {
            src_map
                .entry(area.src.name())
                .or_insert_with(|| (area.src.read().unwrap(), vec![]))
                .1
                .push((area.span, msg));
        }

        let mut colors = RainbowColorGenerator::new(345.0, 0.75, 1.0, 45.0);
        let theme = Theme {
            chars: ThemeChars {
                side_vertical_dotted: '·',
                ..Default::default()
            },
            ..Default::default()
        };

        for (src, (code, labels)) in src_map {
            writeln!(
                f,
                "{}{}{}\n",
                "[".dimmed(),
                src.truecolor(123, 184, 255),
                "]".dimmed(),
            )?;
            write!(
                f,
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
            writeln!(f)?;
        }

        Ok(())
    }
}

#[rustfmt::skip]
macro_rules! error_maker {
    (
        #[title = $title:literal]
        $(
            #[extra_args = ( $($extra:ident: $extra_typ:ty),+ $(,)? )]
        )?
        // $(
        //     #[extra_fields = ( $($extra_field:ident: $extra_field_typ:ty),+ $(,)? )]
        // )?
        $(#[$($meta:tt)*])*
        $name:ident {
            $(
                #[msg = $msg:literal]
                $(
                    #[note = $note:expr]
                )?
                #[labels = [
                    $(
                        $area:expr => $fmt:literal $(: $($v:expr),+ )?;
                    )*
                ]]
                $variant:ident {
                    $(
                        $field:ident: $field_typ:ty,
                    )*
                },
            )*
        }
    ) => {
        #[derive(Debug)]
        $(#[$($meta)*])*
        pub enum $name {
            $(
                $variant {
                    $(
                        $field: $field_typ,
                    )*
                },
            )*
        }

        use $crate::error::ErrorReport;
        impl $name {
            pub fn into_report(self $($(, $extra: $extra_typ)+)?) -> ErrorReport {
                use colored::Colorize;

                let (msg, labels) = match self {
                    $(
                        $name::$variant { $($field,)* } => (
                            $msg,
                            vec![$(
                                ($area, format!($fmt $($(,
                                    $v.to_string().bright_white()
                                )+)?)),
                            )*]
                        ),
                    )*
                };
        
                ErrorReport {
                    title: $title,
                    msg,
                    labels,
                }
            }
        }
    };
}

pub(crate) use error_maker;
