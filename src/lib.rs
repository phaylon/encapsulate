//! Isolates a block of code from surrounding control flow.
//!

#[macro_export]
macro_rules! encapsulate {
    ($($body:tt)*) => {
        $crate::_internal_run(#[inline(always)] || { $($body)* })
    }
}

#[macro_export]
macro_rules! encapsulate_fn {
    ($($body:tt)*) => {
        $crate::_internal_run(#[inline(never)] || { $($body)* })
    }
}

#[macro_export]
macro_rules! encapsulate_flexible {
    ($($body:tt)*) => {
        $crate::_internal_run(|| { $($body)* })
    }
}

#[inline(always)]
#[doc(hidden)]
pub fn _internal_run<F, R>(body: F) -> R where F: FnOnce() -> R { body() }
