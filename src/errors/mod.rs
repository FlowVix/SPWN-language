pub mod diagnostic;
pub mod emitter;

pub use diagnostic::*;
pub use emitter::*;
use macro_pub::macro_pub;

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
                // $(
                //     #[note = $note:expr]
                // )?
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

        impl Into<$crate::errors::Diagnostic> for $name {
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
                            message: $msg.into(),
                            suggestions: vec![],
                        },
                    )*
                }
            }
        }
    };
}
