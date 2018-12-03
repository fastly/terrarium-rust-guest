//! Panic handler for running within a guest.
//!
//! Based on code from the
//! [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
//! crate.

use std::panic;

use hostcalls::panic_hook;

/// A panic hook for use with the demo hostcall interface.
pub fn hook(info: &panic::PanicInfo) {
    panic_hook(&info.to_string());
}

/// Set the hostcall panic hook the first time this is
/// called; subsequent invocations do nothing.
///
/// This is meant to be used by the `guest_app` scaffolding macro, but
/// is harmless to call directly. We hide the doc so that users aren't
/// encouraged to mess with it, but it needs to be exported so the
/// macro can work.
#[doc(hidden)]
#[inline]
pub fn panic_set_once() {
    use std::sync::{Once, ONCE_INIT};
    static SET_HOOK: Once = ONCE_INIT;
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(hook));
    });
}
