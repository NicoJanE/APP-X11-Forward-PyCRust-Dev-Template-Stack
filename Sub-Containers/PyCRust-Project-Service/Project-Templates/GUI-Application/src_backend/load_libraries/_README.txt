Python module to load the specific Rust libraries

Contains code to open Rust libraries. Each specific Rust library has its own file, which includes the code 
for opening and closing the library and defining the API interface (wrapper) that can be called by other 
programs (e.g., Python).

    * rust_lib_core.py: A specific library file used to load core_lib.
    * generic_rust_library.py: Contains low-level implementation code to facilitate the loading of specific Rust libraries.