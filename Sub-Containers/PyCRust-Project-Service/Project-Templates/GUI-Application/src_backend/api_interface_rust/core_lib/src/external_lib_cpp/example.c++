// your_source.cppb
#include "headers/example.h"

std::unique_ptr<MyCppClass> new_cpp_class() {
    return std::make_unique<MyCppClass>();
}
