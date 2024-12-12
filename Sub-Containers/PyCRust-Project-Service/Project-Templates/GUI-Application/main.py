# Add the project root to sys.path
import sys
import os
#sys.path.append(os.path.abspath(os.path.dirname(__file__)))

# Projet imports
from project_constants import OS

# Package and module imports
# --------------------------------------------------------------
#       Package = directory
#       module  = .py file
#       __init__.py: contains rust_library and exported members(OpenLib)
#
#
from    src_backend.load_libraries import rust_lib_core as lib_core   # Method 1 Module import, the __init__.py is used to import module items (*preferred)
#                                                                      #   USAGE: lib_instance = Rust.OpenLib("libcorelib", "release",cdef_interface).get_Library()
#
# import src_business.api_interface.loadlibrary.rust_library    # Method 2 Using Full Module Import, all items in the: 'rust_library.py' module file are imported
#                                                               # You also always need to fully reference the full module path!
#                                                               #   USAGE: lib_instance = src_business.loadlibrary.rust_library.IRustLibrary("libcorelib", "release",cdef_interface).get_Library()
#                                                               
#
# import  src_business.api_interface.loadlibrary                # Method 3 All  package items defined in the __init__.py is can be accessed through 'src_business.loadlibrary'
#                                                               # Items not in __init__.py are still  accessible  but must be referenced with the full qualified name,
#                                                               #   EXAMPLE:  'src_business.loadlibrary.functionx'
#
# --------------------------------------------------------------
 

from ctypes import CDLL
# lib = CDLL("./src_backend/api_interface_rust/lib_core/target/libs/lib_ext_cpp.so")  # Replace with your library's path

# Open the Rust library
Rustcore_lib =lib_core.Open_libcorelib()

# Call two Rust library functions
add_result = Rustcore_lib.add(5, 2600150)
multiply_result = Rustcore_lib.multiply(10, 4)
print(f"Result of Rust library function (addition): {add_result}")
print(f"Result of Rust library function (multiplication): {multiply_result}")



# Sample 1 external library.
# of calling an external C++ library via the Rust library (perhaps not so opaque after all).
# prints 'Hello from C++!! from the external library, which was called via the Rust library"
#Result = Rustcore_lib.elib_open_external_lib(b"test_lib1")     
#result = Rustcore_lib.elib_hello_world(b"test_lib1")
#result = Rustcore_lib.elib_close_external_lib(b"test_lib1")       



# FIXME: Even if closed can only can not reopen
# Second time calling(open) the library fails
# The issue you're encountering is likely due to the behavior of the libloading crate in Rust. When you 
# close a library (e.g., by dropping it or removing it from your HashMap), the library's resources are unloaded 
# from memory. However, some systems or runtime environments do not fully reinitialize the library upon reopening. 
# Here are some potential causes and solutions:
#Result2 = Rustcore_lib.elib_open_external_lib(b"test_lib2")     
#result2 = Rustcore_lib.elib_hello_world(b"test_lib2")
#result2 = Rustcore_lib.elib_close_external_lib(b"test_lib2")       

# FIXME: Even if closed can only can not reopen
# Sample 2 external library.
# Open the external library implicite by calling into the Rust library
Rustcore_lib.call_external_from_rust_lib()
import time
time.sleep(2)
Rustcore_lib.call_external_from_rust_lib()