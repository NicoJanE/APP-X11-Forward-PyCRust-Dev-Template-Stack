from project_constants                                 import OS
from src_backend.load_libraries     import generic_rust_library as Rust   
from src_backend.load_libraries     import libpaths

# Load the ***specific*** Rust module libcore_lib (see package:corelib)
# Defined in api_interface_rust/core_lib
# Usage:
#           from src_business.loadlibrary import rust_lib_core as lib_core
#           Rustcore_lib =lib_core.Open_libcorelib()
#           add_result = Rustcore_lib.add(5, 2900150)
#
# Note: functions starting with: elib_ are function calls into an external library ('libmy_cpp_lib') via this library.
#
def Open_libcorelib():
    
    # The Rust interface methods to import, functions  starting with elib_ are calling an other external library
    Rust_API_members = """
        int add(int a, int b);
        int multiply(int a, int b);        
        void call_external_from_rust_lib();
        int elib_open_external_lib(const char*);
        int elib_hello_world(const char*);
        int elib_close_external_lib(const char*);
        
    """

    # this opens the rust library
    Rustcore_lib = Rust.OpenLib("libcore_lib", "debug",Rust_API_members, libpaths).get_Library()
    return Rustcore_lib