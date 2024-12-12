// This file: mod.rs is the main file of 'lib_external_lib_loader'

mod imp_library_loader;                    // Implementation, lower abstracton level functions 
    use imp_library_loader as LL;          // Alias 

//System uses    
use std::ffi::CStr;
use std::os::raw::c_char;    

// The name of the external C++ library used
const LIBRARY_NAME: &str            = "lib_ext_cpp";    


// Result codes
const OKAY: i32 = 0;
const ERR_CALL_LIB_FUNCTION: i32    = -98;
const ERR_OPEN_LIBRARY: i32         = -99;
const ERR_CLOSE_LIBRARY: i32        = -100;
const ERR_CONVERSION: i32           = -500;







// BEGIN. Public wrappers
// ======================================================================================================
//
// General: this section contains wrapper C functions for the external library: 'lib_ext_cpp'
// All wrappers are public and start with: elib_ follewed by a name.
//


/// Call this function  to open the external C++ library 'libmy_cpp_lib.so'. C wrappers for this library are available in this module. The
/// wrappers start with elib_[name] 
/// After calling all wrappers use: close_external_lib() to close this library. Note that you can call this external(i.e Python), but also
/// in the library itself.
/// 
/// Returns: 0 on success
/// returns: -99 in case the library could not be openened
#[no_mangle]
pub extern "C" fn elib_open_external_lib(library_id: *const c_char) -> i32 
{
    let mut lib_handle = LL::LIB_HANDLE.lock().unwrap();            // Auto-lock mutably to modify LIB_HANDLE, lib_handle is a reference to ourlazy static!
    if lib_handle.is_none() 
    {    
        // Convert the: library_id
        let c_str = unsafe { CStr::from_ptr(library_id) };
        let library_id_str = match c_str.to_str()
        {
            Ok(s) => s,
            Err(_) => return ERR_CONVERSION,                        // Return an error if the conversion fails  
        };

        // Detect OS and add extension
        let lib_name_ext = append_library_extension(LIBRARY_NAME);
    
        let mut manager = LL::LibraryLoader::new();   
        let library_locations = manager.get_library_path(lib_name_ext.as_str());
        for path in library_locations 
        {
            if let Some(path_str) = path.to_str() 
            {
                if manager.open(library_id_str, path_str).is_ok() 
                {
                    *lib_handle = Some(manager);                    // After opening the library successfully, store the manager inside the static Mutex Store(LIB_HANDLE)
                    println!("cargo:warning=Debug       PROGRAM INFO:       - External library is openend ");
                    return OKAY;
                }
            }
        }         
    }    
    println!("cargo:warning=Debug       PROGRAM INFO:       - External library could **not** be openend! ");
    return ERR_OPEN_LIBRARY;         
}


/// Call this function to close the external C++ library 'lib_ext_cpp_lib'
#[no_mangle]
pub extern "C" fn elib_close_external_lib(library_id: *const c_char) -> i32
{
    // Convert the: library_id
    let c_str = unsafe { CStr::from_ptr(library_id) };
    let library_id_str = match c_str.to_str()
    {
        Ok(s) => s,
        Err(_) => return ERR_CONVERSION,                                    // Return an error if the conversion fails  
    };
    
    // Close Library if opened
    let mut lib_handle = LL::LIB_HANDLE.lock().unwrap();                    // Lock and access the library safely 
    if let Some(manager) = &mut *lib_handle                                 // option object has 'Some' valid value, library open       
    {
        {
            let _err = manager.close(library_id_str);
            println!("\ncargo:warning=Debug       PROGRAM INFO:      - Library closed, thanks! No sure if really closed(can not open again)\n");                       
            return OKAY; 
        }
    } 
    return ERR_CLOSE_LIBRARY;
}


/// A simple function which calls the hello_world function in  the C++ library: 'libmy_cpp_lib'
/// 
/// Returns: 0 on success
/// returns: ERR_CALL_LIB_FUNCTION(-98) in case of a call failure or ERR_CONVERSION(-200)
#[no_mangle]
pub extern "C" fn elib_hello_world(library_id: *const c_char)  ->i32
{         
    // Call the function in an opened library
    let lib_handle = LL::LIB_HANDLE.lock().unwrap();            // Lock and access the library safely
    match &*lib_handle
    { 
        Some(manager) =>                                        // Option object has 'Some' valid value, library open       
        {            
            // Convert the: library_id
            let c_str = unsafe { CStr::from_ptr(library_id) };
            let library_id_str = match c_str.to_str()
            {
                Ok(s) => s,
                Err(_) => return ERR_CONVERSION,                // Return an error if the conversion fails  
            };

            //println!("cargo:warning=Debug       PROGRAM INFO:       - Library open, try to call function\n");           
            if let Err(_err) = manager.call_function(library_id_str, "Call") 
            {
                println!("Error: {}", _err);
                return ERR_CALL_LIB_FUNCTION;  
            }          
        }
        None =>
        {
            println!("\n\n\t**WARNING**\n\n\tLibrary is not explicitly opened by YOU. You should open it, by calling open_lib() from the client!!\n\tAfter using it use: free_library() from the client.");
            println!("\t** Rest of code is aborted !\n\n\n");        
        }      
    }    
    return OKAY;
}


// ------------------------------------------------------------------------------------------------------
// END. Public wrappers


// BEGIN implementation functions and Helpers 
// ======================================================================================================

fn append_library_extension(base_name: &str) -> String 
{
    // Detect OS using cfg! Marco (compile time) alternative: std::env::consts::OS (runtime)
    let mut lib_name_ext = base_name.to_string();
    if cfg!(target_os = "windows") {
        lib_name_ext.push_str(".dll");
    } else if cfg!(target_os = "linux") {
        lib_name_ext.push_str(".so");
    } else if cfg!(target_os = "macos") {
        lib_name_ext.push_str(".dylib");
    } else {
        eprintln!("Unsupported OS!");
    }

    lib_name_ext
}


// ------------------------------------------------------------------------------------------------------
// End Helpers