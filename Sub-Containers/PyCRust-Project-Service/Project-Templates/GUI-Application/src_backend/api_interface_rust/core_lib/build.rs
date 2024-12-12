use std::env;
use std::process::Command;
use std::path::Path;
use std::str;
use std::fs;
use std::collections::HashSet;    


/// This is the build file, to create the external C++ library (lib_ext_cpp) which is used by this 
/// Rust library. it uses the crate cxx to trigger. after triggering it will execute the GNU C++ compiler
/// to create the library 




// REMARK:
//           Don't know how this is fro you, but I'm annoyed by the VSC syntaxy highlighting
//          
//           VCS and Rust Syntax Errors in Visual Studio Code
//          
//           Issue:
//           In Visual Studio Code, Rust syntax errors are often detected and highlighted 
//           in real-time as you type. However, there's a caveat: if you make a correction 
//           to the code but **do not save the document**, the syntax errors will **persist** 
//           until the file is saved. This can be highly frustrating because:
//          
//           - Even after finding the solution to the error, the syntax highlighting may still 
//             indicate an error, leading you to search for a problem that no longer exists.
//           - The error persists in the editor until the document is saved, even though the 
//             code itself is corrected.
//          
//           Recommendation:
//           To avoid this confusion, **save your file frequently** to ensure that syntax 
//           highlighting and error reporting are up to date. This small habit can help you 
//           avoid unnecessary troubleshooting and improve the overall efficiency of your workflow.
//          
//           Summary:
//           - PROBLEM    Unresolved syntax highlighting until the file is saved.
//           - SOLUTION   Save the file regularly to ensure accurate error detection.
//          
//           - POSSIBLE WORK AROUND
//          {
//              "files.autoSave": "afterDelay",
//              "files.autoSaveDelay": 1000,
//              "rust-analyzer.checkOnSave.enable": true
//          }



const LIBRARY_NAME: &str ="lib_ext_cpp";     
const CPP_FILE_LOCATION: &str ="src/external_lib_cpp/";
const HEADER_FILE_LOCATION: &str ="src/external_lib_cpp/headers";

/// Called via Cargo.toml in combination cxx(bridge.rs)        
fn main() 
{  
    println!("cargo:warning=Debug       PROGRAM INFO:       - Start 'build.rs' for the C/C++ Rust library: 'corelib' ");
    let lib_name = create_dynamic_lib_cpp_code(LIBRARY_NAME);
    println!("cargo:warning=Debug       PROGRAM INFO:{} ",lib_name);
    copy_library(&lib_name); 
} 

fn create_dynamic_lib_cpp_code(lib_name: &str) -> String 
{   
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");    
    let target = env::var("TARGET").unwrap();
    let mut lib_name2 = lib_name.to_string();           // Convert `lib_name` to a mutable

    // Get al c++ files from the CPP_FILE_LOCATION directorie and sub directories that needs to be build
    let cpp_files = get_files(CPP_FILE_LOCATION,"*.c++");
    let cpp_files_str = cpp_files.join(" ");

    // Get al header directory locations, and sub directories based on: HEADER_FILE_LOCATION 
    let header_files = get_directories(HEADER_FILE_LOCATION);
    let header_files_include = generate_include_flags(&header_files);
    println!("cargo:warning=Debug       PROGRAM INFO:       - INCLUDES USED: {}", header_files_include      ); 


    if target.contains("windows")                       //  Manually compile the C++ code using WINDOWS targets (e.g., x86_64-pc-windows-gnu)       
    {        
        lib_name2.push_str(".dll");        
        let cpp_lib_path = Path::new(&out_dir).join(&lib_name2);    // Full path, Borrow out_dir, and lib_name2 (by refrence in C++) =>  &)        

        // Import library file name if required
        let mut import_lib = lib_name.to_string();
        import_lib.push_str("-import");
        let cpp_lib_import = Path::new(&out_dir).join(import_lib).with_extension("lib"); // 

        env::set_var("CXX", "x86_64-w64-mingw32-g++");
        env::set_var("LD", "x86_64-w64-mingw32-ld");
        let cxx = env::var("CXX").unwrap_or_else(|_| "CXX not set".to_string());
        println!("cargo:warning=Debug       PROGRAM INFO:       - CXX is set to: {}", cxx);
        
        let mut status = Command::new("x86_64-w64-mingw32-g++");// Create compile command. Manually compile the C++ code using g++ (Windows,  MinGW)
        status                                                  // fill in the command (hence mut)                                
        .arg("-shared")                                         // Create a shared library
        .arg("-o")                                              // Output flag
        .arg(cpp_lib_path.to_str().unwrap())                    // Output file name (full path)        
        .arg(cpp_files_str)                                     // C++ source files, simple alternative:  .arg("src/external_lib_cpp/example.c++")  
        .arg(header_files_include)                              // Include path to header files
        .arg("-std=c++14")                                      // Use C++14 standard
        .arg("-Wl,--out-implib")                                // Generate an import library for if need, not but generated for convience. This is the Command
        .arg(cpp_lib_import);                                   // Generate an import library for linkingcpp_lib_imoprt. This the file.



        // Let see what the command was
        println!("cargo:warning=Debug       PROGRAM INFO: Command executed (WINDOWS) :");
        println!("cargo:warning=Debug       COMMAND: {}",  format_command(&status));
        println!("cargo:warning=Debug"      );
        
        // Execute the command
        let status = status.status().expect("Failed to run g++");
        if !status.success() 
        {
            panic!("cargo:warning=Debug     PROGRAM FAIL:   - g++ compilation(Windows) failed with error: {:?}", status);        
        } 
        else 
        {        
            println!("cargo:warning=Debug       PROGRAM SUCCESS:    - g++ compilation succeeded(Windows). Shared library generated at: {}", cpp_lib_path.display());    
        }    
    } 
    else if target.contains("linux")                    // For LINUX targets (x86_64-unknown-linux-gnu)
    {                                            
        lib_name2.push_str(".so");
        let cpp_lib_path = Path::new(&out_dir).join(&lib_name2); // Generate full path

        env::set_var("CXX", "/usr/bin/g++");
        env::set_var("LD", "/usr/bin/ld");
        let cxx = env::var("CXX").unwrap_or_else(|_| "CXX not set".to_string());
        println!("cargo:warning=Debug       PROGRAM INFO:       - CXX is set to: {}", cxx);

        let mut status = Command::new("g++");               // Create compile command. Manually compile the C++ code using g++ (Linux)         
        status                                              // fill in the command                
        //.env("CXX", "usr/lib/gcc/x86_64-linux-gnu/13")    // Specify the toolchain if requied
        .arg("-fPIC")                                       // Generate position-independent code
        .arg("-shared")                                     // Create a shared library
        .arg("-o")                                          // Output flag
        .arg(cpp_lib_path.to_str().unwrap())                // Output file name (full path)
        .arg(cpp_files_str)                                 // C++ source files, simple alternative:  .arg("src/external_lib_cpp/example.c++")  
        .arg(header_files_include)                          // Include path to header files
        .arg("-std=c++14");                                 // Use C++14 standard


        // Let see what the command was                
        println!("cargo:warning=Debug       PROGRAM INFO: Command executed (LINUX) :");
        println!("cargo:warning=Debug       COMMAND: {}",  format_command(&status));
        println!("cargo:warning=Debug"      );

        // Execute the command
        let status = status.status().expect("Failed to run g++");
        if !status.success() 
        {
            panic!("cargo:warning=Debug     PROGRAM FAIL:   - g++ compilation(Linux) failed with error: {:?}", status);                    
        } 
        else 
        {        
            println!("cargo:warning=Debug       PROGRAM SUCCESS:    - g++ compilation succeeded(Linux). Shared library generated at: {}", cpp_lib_path.display());                
        }
        
    } 
    else 
    {
        panic!("Unsupported target platform: {}", target);
    }

    // Ensure the C++ files trigger a rebuild if they change
    println!("cargo:rerun-if-changed=src/external_lib_cpp/example.c++");
    println!("cargo:rerun-if-changed=src/external_lib_cpp/example.h");  //println!("cargo:rustc-link-search=native=/usr/lib/gcc/x86_64-linux-gnu/13"); // Ad just as needed
    lib_name2                                                           // without ; indicates a return value     
    
}

// formats the command in create_dynamic_lib_cpp_code(...)
fn format_command(command: &Command) -> String {
    let args = command
        .get_args()
        .map(|arg| arg.to_string_lossy().to_string())
        .collect::<Vec<String>>()
        .join(" ");

    format!("{} {}", command.get_program().to_string_lossy(), args)
}

/// Returns all: *.c++ files in the directorie and in the sub directories
fn get_files(location: &str, file_type: &str) -> Vec<String> {        
    //let find_command = "find ".to_string() + location + " -name '*.c++'";         
    let find_command = "find ".to_string() + location + " -name '"+file_type+"'";   //  CMD: "find src/external_lib_cpp/ -name '*.c++'";       
    println!("cargo:warning=Debug       PROGRAM INFO:       - FIND: {}", find_command      ); 
    let files_output = Command::new("sh")                                           // Execute the command to get the file list
        .arg("-c")
        .arg(find_command)
        .output()
        .expect("Failed to execute find command");
    
    if !files_output.status.success() {
        panic!("find command failed with error: {:?}", files_output);
    }

    // Convert the output to a vector of strings (file paths)
    let files = String::from_utf8_lossy(&files_output.stdout)
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    // println!("cargo:warning=Debug       PROGRAM DBG:       - FILES: {}", files.join(" ")      ); 
    files
}


fn get_directories(location: &str) ->  HashSet<String> 
{        
    //let find_command = "find ".to_string() + location + " -type d ";
    let find_command = format!("find {} -type d", location);  // Ensures we are finding directories recursively    
    println!("cargo:warning=Debug       PROGRAM DBG:        - FIND (DIR.): {}", find_command      ); 
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(find_command)
        .output()
        .expect("Failed to execute find command");
    
    // Collect directories, filtering out the ones we need
    let directories: HashSet<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.to_string())   // Convert each line to a String
        .collect();                     // HashSet automatically removes duplicate
    
    
    //let directories_vec: Vec<String> = directories.clone().into_iter().collect();       // DBG: Convert HashSet to Vec to use `join`
    //println!("cargo:warning=Debug       PROGRAM DBG:       - DIR: {}", directories_vec.join(" ")      ); 
    directories
}

// Uses the get_directories(...) to add g++ Include flags to each directory 
fn generate_include_flags(directories: &HashSet<String>) -> String {
    directories.iter()
        .map(|dir| format!("-I {}", dir))
        .collect::<Vec<String>>()
        .join(" ")
}


// Copies resulting library binary to convient location
fn copy_library(lib_name: &str) 
{    
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let source_path = Path::new(&out_dir).join(lib_name);           
    let target_dir = std::env::var("CARGO_TARGET_DIR")
    .unwrap_or_else(|_| "target".to_string()); 
    let target_dir = Path::new(&target_dir).join("libs/").join(lib_name);

    fs::create_dir_all(target_dir.parent().expect("No parent directory"))
        .expect("Failed to create target directory");

    // Copy the file
    fs::copy(&source_path, &target_dir)
        .unwrap_or_else(|err| panic!("Failed to copy {:?} to {:?}: {:?}", source_path, target_dir, err));

    
    println!("cargo:warning=Debug       PROGRAM INFO:       - Copy the resulting Lib to central location: {}", target_dir.display());        
    eprintln!("cargo:rerun-if-changed={}", source_path.display());   // Tell Cargo to rerun this script if the source file changes
}




// Notes
// I want to be able to switch between Ming32 and Linux, if possible in the build.rs file can we set those explicitly (CXX and LD)
// cargo clean
// cargo build --target x86_64-unknown-linux-gnu

//1) Okay After unset CXX then run:
//cargo build --target x86_64-unknown-linux-gnu
//All good!
//
//2) Run task from VSC (tasks.json)
//Fails. checking $CXX is empty. Doing again : cargo build --target x86_64-unknown-linux-gnu (command line)
//AlL good
//
//3) cargo clean
//cargo build --target x86_64-unknown-linux-gnu
//All good
//
//4 Remove target folder from VSC project (eff. Clean)
//cargo build --target x86_64-unknown-linux-gnu
//all Good