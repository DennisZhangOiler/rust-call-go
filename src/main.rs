mod ffi {
    #![allow(
        unused,
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
        // Silence "128-bit integers don't currently have a known stable ABI" warnings
        improper_ctypes,
        // Silence "constants have by default a `'static` lifetime" clippy warnings
        clippy::redundant_static_lifetimes,
        // https://github.com/rust-lang/rust-bindgen/issues/1651
        deref_nullptr,
    )]
    include!("./bindings.rs");
}

use ffi::SerializeStrings;
use std::ffi::CString;

fn main() {
    let a = CString::new("hello").expect("CString::new failed");
    let b = CString::new(" world").expect("CString::new failed");
    let c = CString::new("!").expect("CString::new failed");

    let d = unsafe { SerializeStrings(a.into_raw(), b.into_raw(), c.into_raw()) };
    let result = unsafe { CString::from_raw(d) };
    println!("{:?}", result);
}
