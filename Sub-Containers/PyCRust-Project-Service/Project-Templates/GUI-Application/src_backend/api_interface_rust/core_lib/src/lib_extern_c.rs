// Module containing the Rust library.
// 


// System 
//      - Make sure to use explicitly imports instead of VSC implicit magic garbage! (suggestions)
//
use std::os::raw::c_char;    
use std::ffi::CString;

// Reference alias to interface to external Library functions
//
use crate::loader_external_lib_interface as I;  



// BEGIN. Public exposed Rust functions (usin C interface) 
// ======================================================================================================================
//

/// Example 1 of a Rust function exported as a C function. Defined in the Rust Library ('libcorelib')
#[no_mangle]
//pub extern "C" fn add2(a: i32, b: i32) -> Result<i32, String>
//{   
//    if a+b >0 
//    { 
//        Ok(a+b)
//    }
//    else
//    {
//        Err("less then zero".to_string())
//    }
//}

pub extern "C" fn add(a: i32, b: i32) -> i32
{        
    a+b        
}



/// Example 2 of a Rust function exported as a C function. Defined in the Rust Library ('libcorelib')
#[no_mangle]
pub extern "C" fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Call a function from the external C++ library, open the library, call the function, close the library 
/// (see:/loader_external_lib_interface/mod.rs for details)
#[no_mangle]
pub extern  "C" fn call_external_from_rust_lib()
{
    // We need to makeup an unique 'library id' as const char 
    let library_id = CString::new("Lib_internal_ID").expect("CString::new failed");     // null terminiated CString
    let library_id_ptr: *const c_char = library_id.as_ptr();                            // *const char => (*const i8)

    let mut _result  = I::elib_open_external_lib(library_id_ptr);
   _result = I::elib_hello_world(library_id_ptr);
   _result = I::elib_close_external_lib(library_id_ptr);   
}


// ------------------------------------------------------------------------------------------------------------------------
// END. Public exposed Rust functions (using C interface) 


// BEGIN. Implementation functions
// ======================================================================================================================
//


// ------------------------------------------------------------------------------------------------------------------------
// END. Implementation functions