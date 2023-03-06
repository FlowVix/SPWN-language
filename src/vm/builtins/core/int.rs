use crate::vm::builtins::builtin_utils::impl_type;
use crate::vm::error::RuntimeError;
use crate::vm::value::{StoredValue, Value};
use crate::vm::value_ops;

impl_type! {
    // most of these are implemented in spwn in the std because Context stuff is skrunly to make in rust
    impl Int {
        Constants:

        Functions(vm, call_area):

        fn abs(Int(n) as self) -> Int {
            Value::Int(n.abs())
        }
        fn sign(Int(n) as self) -> Int {
            Value::Int(n.signum())
        }
        fn sqrt(Int(n) as self) -> Float {
            Value::Float((n as f64).sqrt())
        }
        fn log(Int(n) as self, Float(base) as base = {2.7182818284590452353602874713527}) -> Float {
            Value::Float((n as f64).log(base))
        }
        fn clamp(Int(n) as self, Int(min) as min, Int(max) as max) -> Float {
            Value::Int(n.max(min).min(max))
        }
        fn wrap(Int(n) as self, Int(min) as min, Int(max) as max) -> Float {
            Value::Int(((n - min) % (max - min)) + min)
        }
    }
}
