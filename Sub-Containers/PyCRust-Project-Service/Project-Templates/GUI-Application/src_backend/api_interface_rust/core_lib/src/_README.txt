The actual Rust 'lib_core' library files, and the code to load the external CPP library(loader_external_lib_interface) from the lib_core library

    -   The 'bridge.rs' file is needed to create the C++ lib calls '../build.rs' (using cxx)
    -   The 'loader_external_lib_interface.rs' Loads the external C++ library and contains the C wrappers for the C++ code. 
        Inside this directory module:
        
            - 'mod.rs':                 The highlevel function to open the external library and call the C wrappers.
            - imp_library_loader.rs:    Low level implementation file.
        
    -   The 'lib_extern_.rs' The Rust library functions with and without extern C interface.
    -   The 'lib.rs' This is the main library function.