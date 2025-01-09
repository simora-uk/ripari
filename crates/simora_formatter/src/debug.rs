use std::sync::atomic::{AtomicBool, Ordering};

static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn set_verbose(verbose: bool) {
    VERBOSE.store(verbose, Ordering::SeqCst);
}

pub fn debug_println(msg: &str) {
    if VERBOSE.load(Ordering::SeqCst) {
        println!("{}", msg);
    }
}

pub fn debug_format(fmt: std::fmt::Arguments<'_>) {
    if VERBOSE.load(Ordering::SeqCst) {
        println!("{}", fmt);
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        $crate::debug::debug_format(format_args!($($arg)*));
    }};
}
