use std::fmt::Display;

pub trait SafeUnwrap {
    fn safe_unwrap(self);
}

pub trait UnwrapWithTip: SafeUnwrap {
    fn unwrap_with_tip(self, tip: &str);
}

impl<E: Display> SafeUnwrap for Result<(), E> {
    fn safe_unwrap(self) {
        if let Err(e) = self {
            println!("{e}")
        }
    }
}

impl<E: Display> UnwrapWithTip for Result<(), E> {
    fn unwrap_with_tip(self, tip: &str) {
        let is_err = self.is_err();
        self.safe_unwrap();
        if is_err { println!("Tip: {tip}"); }
    }
}
