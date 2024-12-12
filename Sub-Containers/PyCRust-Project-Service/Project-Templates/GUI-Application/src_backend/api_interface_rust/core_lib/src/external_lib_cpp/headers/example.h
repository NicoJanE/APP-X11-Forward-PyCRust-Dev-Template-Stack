// example.h
#pragma once

#include <memory>
#include <iostream>

class MyCppClass {
public:
    MyCppClass() = default;
    void greet() const {
        std::cout << "\t- Hello from C++!! from the external library, which was called via the Rust library" << std::endl;
    }
};

// Free function Returns instance of the class Required by cxx for handling smart pointers
std::unique_ptr<MyCppClass> new_cpp_class();

extern "C" __attribute__((visibility("default")))  void Call()
{
  std::unique_ptr<MyCppClass> obj = std::make_unique<MyCppClass>();   
  obj->greet();
}

