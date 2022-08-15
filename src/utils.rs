use std::os::raw::{c_int};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[link(name = "add")]
extern{
    fn add(a: u32, b: u32) -> u32;
}

pub fn add_from_c(a: u32, b: u32) -> u32 {
    unsafe {
        return add(a, b);
    }
}
