Libraries created in Rust with a public interface. 
    *  lib_core
        -   This library contains the core functionality defined in Rust, which can be called from other programs.
        -   It also defines a C++ external library that is created during the build process of 'lib_core'. library. 
            -   The C++ external library can be invoked within lib_core itself or accessed through lib_core from other 
                programs (e.g., Python). This allows external programs to directly call functions from the C++ library 
                via lib_core.
    *  lib_tool
        - A sample library without real functionality, included for demonstration or testing purposes.
