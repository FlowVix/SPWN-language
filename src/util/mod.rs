use std::borrow::Cow;

use regex::Regex;

pub mod slabmap;
pub mod unique_register;

pub type ImmutCloneStr = std::rc::Rc<str>;
pub type ImmutStr = Box<str>;
// pub type ImmutCloneStr32 = Rc<Str32>;
// pub type ImmutStr32 = Box<Str32>;
pub type ImmutCloneVec<T> = std::rc::Rc<[T]>;
pub type ImmutVec<T> = Box<[T]>;

/// all values in range `0-1`
pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let h = h * 6.0;

    let c = v * s;
    let x = c * (1.0 - (h.rem_euclid(2.0) - 1.0).abs());

    let (r, g, b) = if (0.0..1.0).contains(&h) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&h) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&h) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&h) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&h) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let m = v - c;
    let (r, g, b) = (r + m, g + m, b + m);

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

lazy_static::lazy_static! {
    static ref ANSI_REGEX: Regex = Regex::new(r#"(\x9B|\x1B\[)[0-?]*[ -/]*[@-~]"#).unwrap();
}

pub fn clear_ansi(s: &str) -> Cow<'_, str> {
    ANSI_REGEX.replace_all(s, "")
}
