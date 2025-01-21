#include <dlfcn.h>  // For dynamic linking

#include <iostream>

typedef int (*AddNumbersFn)(int, int);
typedef const char* (*GreetFn)();

int main() {
    // Load the Rust shared library
    const char* file_path =
        "/home/tourist/diy/LearnRust/target/release/librust_lib.so";
    void* handle = dlopen(file_path, RTLD_LAZY);
    if (!handle) {
        std::cerr << "Error loading library: " << dlerror() << std::endl;
        return 1;
    }

    // Get the `add_numbers` function
    auto add_numbers = (AddNumbersFn)dlsym(handle, "add_numbers");
    if (!add_numbers) {
        std::cerr << "Error loading function add_numbers: " << dlerror()
                  << std::endl;
        dlclose(handle);
        return 1;
    }

    // Get the `greet` function
    auto greet = (GreetFn)dlsym(handle, "greet");
    if (!greet) {
        std::cerr << "Error loading function greet: " << dlerror() << std::endl;
        dlclose(handle);
        return 1;
    }

    // Use the functions
    int result = add_numbers(5, 3);
    std::cout << "5 + 3 = " << result << std::endl;

    const char* message = greet();
    std::cout << message << std::endl;

    // Close the library
    dlclose(handle);
    return 0;
}
