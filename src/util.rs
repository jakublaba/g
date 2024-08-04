use std::fmt::Display;

pub trait SafeUnwrap {
    fn safe_unwrap(self);
}

impl<E: Display> SafeUnwrap for Result<(), E> {
    fn safe_unwrap(self) {
        if let Err(e) = self {
            println!("{e}")
        }
    }
}
