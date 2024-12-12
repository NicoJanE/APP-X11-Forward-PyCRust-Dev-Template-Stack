// bridge.rs
#[cxx::bridge]
pub mod ffi {
    // Define the C++ class interface.
    unsafe extern "C++" {
     
         // Include the C++ header file path.
         // Required to inform `cxx` to invoke `build.rs` for compiling the C++ code.
         //
         // In `build.rs`, we don't use the default `cxx` approach for C++ code compilation.
         // Instead, we use `Command::new` to call the GCC compiler directly, creating a dynamic 
         // library with a C++ wrapper. This wrapper provides an `extern "C"` function exported 
         // from the library, which Rust can call to interact with the C++ code.
         //
         // Note: `build.rs` runs when this file is saved. Therefore, a clean and rebuild operation
         // without saving `bridge.rs` may not regenerate the C++ dynamic library, as the C++ 
         // compilation in `build.rs` is triggered specifically by saving this file.
         // This has been fixed by 'touching' this file in VSC when before the build is executed
        include!("example.h");              //  include!(concat!(env!("CARGO_MANIFEST_DIR"), "/cpp/example.h"));
    }
}

