
use libloading::{Library, Symbol};
use std::{env, path::PathBuf, sync::Mutex};
use lazy_static::lazy_static;
use std::collections::HashMap;


// Global static variable to hold the library, wrapped in a Mutex to ensure thread-safety
lazy_static! {
    pub(crate) static ref LIB_HANDLE: Mutex<Option<LibraryLoader>> = Mutex::new(None);
    
}


// Defiintion LibraryLoader
pub(crate) struct LibraryLoader 
{
    libraries: HashMap<String, Library>,                    // Map library names to handles 
}

// Implementation structure LibraryLoader
impl LibraryLoader 
{
    pub fn new() -> Self 
    {
        Self { libraries: HashMap::new(), }
    }

     /// Open a library and associate it with a name
     pub fn open(&mut self, name: &str, path: &str) -> Result<(), String> 
     {
        match unsafe {Library::new(path)} 
        {
            Ok(lib) => 
            {                
                self.libraries.insert(name.to_string(), lib);                
                Ok(())
            }
            Err(err) => Err(format!("Failed to open library '{}': {}", name, err)),
        }
    }


    /// Call a function from a specified library
    pub fn call_function(&self, lib_name: &str, func_name: &str) -> Result<(), String> 
    {        
        if let Some(lib) = self.libraries.get(lib_name) 
        {     
            unsafe 
            {
                let symbol: Symbol<unsafe extern "C" fn()> = lib
                    .get(func_name.as_bytes())
                    .map_err(|_| format!("Function '{}' not found in library '{}'", func_name, lib_name))?;
                symbol();
                Ok(())
            }
        }
        else 
        {
            Err(format!("Library '{}' not loaded", lib_name))
        }
    }
    

    // Call:     xxx.get_library_path("libexample.so");
    pub fn get_library_path(&self, library_name: &str ) ->  Vec<PathBuf>
    {
        let mut lib_build_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        lib_build_path.push("target/libs/");
        let library_paths = vec![
            PathBuf::from(format!("./{}", library_name)),                             // Load from Release location (root directory)
            PathBuf::from(format!("{}{}", lib_build_path.display(),library_name)),    // Load from Build location(debug)
            
        ];

        library_paths
    }

    /// Close the library
    pub fn close(&mut self, name: &str) -> Result<(), String> 
    {        
        if let Some(library) =self.libraries.remove(name)
        {
            // REMARK: FIXME:  libloading::Library defers library by default.  Forcefully drop the library to unload it. DOES NOT WORK of course            
            //  Solution se this link: https://users.rust-lang.org/t/how-to-avoid-library-and-symbol-drops-in-crate-libloading/85701
            //      first conclusion: 'unloading a library is hard.' (related lazy static, but anyway unre)
            //          option in open: open_already_loaded() ???
            println!("cargo:warning=Debug       PROGRAM INFO:       -  DROP closed Library ");              
            drop(library);
            Ok(())
        } 
        else
        {
            Err(format!("Library '{}' not loaded", name))
        }
    }


}