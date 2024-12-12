from cffi import FFI
import os
import platform

from project_constants import OS            # import project_constants as pc

###     Method Used to Access Rust from Python
# -------------------------------------------------------------------------------------------------------------------------
#       We use cffi (Foreign Function Interface for Python) to create an interface to a Rust library (.so or .dll). 
#       This allows us to access functions in the Rust library by defining a C-compatible function signature for each Rust
#       function we need to call. The cffi module loads the library and, through the defined C interface, we can directly 
#       access our Rust functions.
#
##  Suggested Options for Implementing the cffi C interface
##  1.  Class load Approach
#       Description: This method returns a library interface, allowing access to Rust API functions.
#     
#       When to Use: 
#       - For Rust libraries that are private to the application or service.
#       - In situations where the library is small and unlikely to change frequently.
#   
#       Properties: 
#       - Static, singleton pattern.
#       - Effective performance with minimal overhead.
#       - Some state maintenance is possible.
#       - Simple implementation.
#       - Supports dependency injection (e.g., library name).
#       - The interface is variable, the API can be defined outside the class
#   
##  2.  Full Class Wrapper
#       Description: This approach wraps the entire Rust API interface within a class, providing a comprehensive object-oriented interface.
#   
#       When to Use:
#       - For Rust API items that are publicly exposed.
#       - When there is a need for future inheritance and extension of the library.
#     
#       Properties:
#       - Instance-based classes allow for more robust state management.
#       - More complex implementation, which may require additional maintenance.
#       - Supports dependency injection (e.g., library name, operating system, release/debug mode).
#       - The interface is fixed, API can not be defined outside the class 
###
 
 

# Using method 1  Class load Approach, with flexible interface
# Loads the and returns the library to the Rust interface up on request
class OpenLib:
    
    def __init__(self, libName, buildType, cdef_interface,libpaths):                        
        self._libName = libName
        self._buildType =  buildType
        self.ffi = FFI()
        self._cdef_interface = cdef_interface
        self._rustLib = None        
        self.libpaths = libpaths
        self.load_Library()


    def load_Library(self):
        self.ffi.cdef(self._cdef_interface)
        lib_win_ext  ="dll"
        lib_lin_ext  ="so"
    
        
        # Define the paths to the libraries (diiferent for Window and Linx )        
        lib_win_path = os.path.join(os.path.dirname(__file__), f"{self.libpaths[OS.WIN]}{self._buildType}/")
        lib_lin_path = os.path.join(os.path.dirname(__file__), f"{self.libpaths[OS.LINUX]}{self._buildType}/")
        print(f"\n- DEBUG: Windows\t\tPATH  library loaded: {lib_win_path}")
        print(f"\n- DEBUG: Linux\t\tPATH  library loaded: {lib_lin_path}\n")

        # Get the library information based on the OS and Release/Debug build
        if platform.system() == "Linux":
            lib_path = f"{lib_lin_path}{self._libName}.{lib_lin_ext}"
            if(self._buildType=="debug"):
                print(f"Trying to load Linux library from: {lib_path}")        
        elif platform.system() == "Windows":
            lib_path = f"{lib_win_path}{self._libName}.{lib_win_ext}"
            if(self._buildType=="debug"):
                print(f"Trying to load Windows library from: {lib_path}")        
        else:
            raise OSError("Unsupported operating system")

        # Load the Rust shared library, Windows or Linux version
        print(f"\n- DEBUG: OS\t\tPATH  library loaded: {lib_path}\n")
        try:
            self._rustLib = self.ffi.dlopen(lib_path)
        except OSError as e:
            print(f"Failed to load library at {lib_path}")
            print(f"Error: {e}") 
            
        if(self._buildType=="debug"):
            print(f"\n- DEBUG\t\tRust library loaded: {self._rustLib}\n")

    # Note you can use the decoration: @classmethod to indicates static class behaviour, but the clas instance implementation here is cleaner
    def get_Library(self):
        if self._rustLib is None:
            raise Exception("RustLibrary instance is not created  or loaded!")
        return self._rustLib

#class OpenLibStatic:
    

# Using method 2 Full Class Wrapper
# Like method  1 but with wrapper methods for the Rust functions.
# The class wrapper can also be implemented by the caller of this class