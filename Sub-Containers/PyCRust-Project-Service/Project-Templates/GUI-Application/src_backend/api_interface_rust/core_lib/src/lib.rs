/// MAIN for the library
/// =================================================================================================================================
/// 
/// Entry point for the Rust library.
///
/// This file serves as the root of the library and contains references to:
/// 
/// -   Module with exported C functions:  
///     References the module defined in `lib_extern_c.rs` that contains the C function bindings.
/// 
/// -   Module for managing external library:  
///     Contains the module that wraps an external library, which can be loaded dynamically.  
///     This module also includes the necessary wrappers for invoking functions from the external library.
/// 
/// -   Support code:  
///     Includes references to helper code that aids in managing the external library interaction and other necessary tasks.
///
/// This file ties together the core functionality, including external integrations and the C function bindings.
///
/// =================================================================================================================================



// The bridge module; Ensures that the external C++ library is compiled and linked
// through `build.rs` (which invokes the C++ compiler).
// 
//      **Note**: This is **not** the traditional CXX bridge method. Instead, I use `build.rs` to compile and 
//      link an external C++ library, which is then made available to the Rust code. This setup involves opening, 
//      closing, and defining wrapper methods for interacting with the C++ library from Rust (see: lib_external_lib_interface).
//      
//      The traditional CXX bridge approach typically compiles C++ directly into the Rust library via `build.rs`, 
//      but in this case, the external C++ library is kept separate.
//
mod bridge;                                          


// Include the Module with public Rust function defined in this Rust library
mod lib_extern_c;

/// Module for managing(opening/closing/using) the external C++ library and related wrappers in Rust.
/// This module contains public functions to open\close the external library, as well as public wrapper
/// functions for the external library
/// 
///     *** Note on file structure in directory 'loader_external_lib_interface' ***
///     As a Rust developer, I find it quite frustrating that I am *forced* to name a file `mod.rs` inside 
///     the directory `lib_external_lib_interface` instead of just naming it `lib_external_lib_interface.rs` 
///     to match the directory. This restriction feels overly rigid, especially when there are alternatives available, 
///     such as:
///     
///     1.  Explicit Path Imports:  
///         This is an awkward workaround, requiring `#[path]` attributes everywhere, which feels clunky and obscure.
///     
///     2.  Flat Structure:  
///         I initially tried a flat structure, but this resulted in having to:
///         - Import `lib_external_lib_loader.rs` directly in `lib.rs` instead of `lib_external_lib_interface.rs`.
///         - Use a `crate` statement (`use crate::lib_external_lib_loader;`) in the interface file, which feels like duct-taping 
///           the module system to work around its own rules.
///     
///     **Conclusion**: It would be much cleaner and more intuitive if we could simply use `lib_external_lib_interface.rs`
///     instead of the `mod.rs` convention. Unfortunately, I have to follow the Rust module system's guidelines here.
/// 
/// 

// Include module with the C++ library functions (open/close/wrappers)
mod loader_external_lib_interface;              //    use loader_external_lib_interface as I;  =>  Optional alias for convenience


// Re-export everything from lib_extern_c. ESSENTIAL to be able to reference in other programs
 pub use lib_extern_c::*;   
pub use loader_external_lib_interface::*; 