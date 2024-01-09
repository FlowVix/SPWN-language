pub mod diagnostic;
pub mod emitter;

pub use diagnostic::*;
pub use emitter::*;
use macro_pub::macro_pub;

use crate::util::hsv_to_rgb;

// #[rustfmt::skip]
// #[macro_pub]
// macro_rules! Diagnostic {
//     (
//         $(#[$($meta:meta)*])*
//         $vis:vis enum $ident:ident
//         {
//             $(
//                 $(#[$($variant_meta:meta)*])*
//                 $variant:ident
//                 {
//                     $($fields:tt)*
//                 }
//             ),*
//             $(,)?
//         }
//     ) => {
//         $(
//             #[
//                 Diagnostic!(#SkipCustomMeta $($meta)*);
//             ]
//         )*
//         $vis enum $ident {

//         }

//         impl Into<$crate::errors::Diagnostic> for $ident {
//             fn into(self) -> $crate::errors::Diagnostic {
//                 todo!()
//             }
//         }
//     };
//     (
//         $(#[$($meta:meta)*])*
//         $vis:vis struct $ident:ident
//         {
//             $($fields:tt)*
//         }
//     ) => {

//     };

//     (
//         #SkipCustomMeta
//         $($meta:meta)*
//     ) => {
//         $(
//             ::defile::defile! {
//                 Diagnostic!(#SkipCustomMeta $(@$meta)*);
//             }
//         )*
//     };

//     (#SkipCustomMeta $m1:ident :: $($rest:tt)*) => {

//     };
//     (#SkipCustomMeta $ident:ident ( $($rest:tt)* ) ) => {
//         Diagnostic!(@IsCustomMeta $ident);
//     };
//     (#SkipCustomMeta $ident:ident = $($rest:tt)*) => {
//         Diagnostic!(@IsCustomMeta $ident);
//     };
//     (#SkipCustomMeta $ident:ident) => {
//         Diagnostic!(@IsCustomMeta $ident);
//     };

//     (@IsCustomMeta $(title)? $(message)? $(label)? $(level)?) => {

//     };
//     (@IsCustomMeta $ident:ident) => {

//     };
// }

// Diagnostic! {
// #[test::a()]
// #[title("Syntax Error")]
// pub enum SyntaxError2 {
//     #[message("Unexpected Token")]
//     UnexpectedToken {
//         expected: String,
//         found: String,
//         #[label("Expected {}, got {}", expected, found)]
//         area: String,
//     },
// }
// }

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

#[rustfmt::skip]
#[macro_pub]
macro_rules! diagnostic {
    (
        #[title = $title:literal]
        #[level = $level:ident]
        $(
            #[extra_args = ( $($extra:ident: $extra_typ:ty),+ $(,)? )]
        )?
        $(#[$($meta:tt)*])*
        $name:ident {
            $(
                #[message = $msg:literal]
                $(
                    #[note = $note:expr]
                )*
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
        
        #[allow(clippy::from_over_into)]
        impl Into<$crate::errors::Diagnostic> for $name {
            #[allow(path_statements)]
            fn into(self) -> $crate::errors::Diagnostic {
                match self {
                    $(
                        $name::$variant { $($field,)* } => $crate::errors::Diagnostic {
                            level: $crate::errors::Level::$level,
                            title: $title.into(),
                            labels: vec![$(
                                (format!($fmt $($(,
                                    $v.to_string()
                                )+)?), $area),
                            )*],
                            notes: vec![$(
                                $note.into(),
                            )*],
                            message: $msg.into(),
                        },
                    )*
                }
            }
        }
    };
}
