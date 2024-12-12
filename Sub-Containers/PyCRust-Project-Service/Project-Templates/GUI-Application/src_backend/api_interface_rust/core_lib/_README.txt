lib_core is a Rust library that exposes public Rust functions for use in Python (also part of this project).
There are two distinct source directories:

1.  ./src:
    This directory contains the actual Rust library code, with 'lib.rs' serving as the entry point.
        * The bridge.rs file is required for 2 (see below)  to build the external library (external_lib_cpp)
        * The compiled Rust library can be found at:
            - src_backend/target/x86_64-unknown-linux-gnu/debug (for Linux)
            - src_backend/target/x86_64-pc-windows-gnu/debug (for Windows)
            
2.  ./src/external_lib_cpp:
    This directory contains the source code for the external library, which is built using cxx and ../build.rs(in root). This external library:
        * Can be used within the Rust library or externally (e.g., from Python, as in this project).
        * After building this C++ library the compiled binary will be copied to: src_backend/api_interface_rust/lib_core/target/libs.
    

