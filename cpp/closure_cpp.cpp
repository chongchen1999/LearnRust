#include <functional>
#include <iostream>
#include <string>

int main() {
    // Example 1: Fn closure equivalent
    // In C++, lambda expressions can capture variables by value, similar to
    // Rust's Fn.
    int x = 5;
    auto square = [x](int num) -> int { return num * x; };
    std::cout << "Square of 4: " << square(4)
              << std::endl;  // Output: Square of 4: 20
    std::cout << "Square of 6: " << square(6)
              << std::endl;  // Output: Square of 6: 30

    // Example 2: FnMut closure equivalent
    // In C++, lambda expressions can capture variables by reference, allowing
    // mutation.
    int y = 10;
    auto increment = [&y](int num) -> int {
        y += num;
        return y;
    };
    std::cout << "Incremented y by 3: " << increment(3)
              << std::endl;  // Output: Incremented y by 3: 13
    std::cout << "Incremented y by 5: " << increment(5)
              << std::endl;  // Output: Incremented y by 5: 18

    // Example 3: FnOnce closure equivalent
    // In C++, lambda expressions can capture variables by move, consuming them.
    std::string z = "1234567890ppppppppppppppppppppppppppppppppppppppp";
    std::cout << "address of z: " << &z << std::endl;
    std::cout << "address of z points to: " << static_cast<const void*>(z.data()) << std::endl;

    // std::string w = std::move(z);
    // std::cout << "address of w: " << &w << std::endl;
    // std::cout << "address of w points to: " << static_cast<const void*>(w.data()) << std::endl;

    auto consume = [](std::string s) -> std::string {
        std::cout << "Consumed string: " << s << std::endl;
        std::cout << "address of s: " << &s << std::endl;
        std::cout << "address of s points to: " << static_cast<const void*>(s.data()) << std::endl;
        return s;
    };
    std::string result = consume(std::move(z));  // z is moved here
    std::cout << "Result: " << result
              << std::endl;  // Output: Consumed string: Hello, Result: Hello
    std::cout << "address of result: " << &result << std::endl;
    std::cout << "address of result points to: " << static_cast<const void*>(result.data())
              << std::endl;

    // Uncommenting the following line would cause a runtime error because z has
    // been moved. 
    // std::cout << "Trying to use z again: " << z << std::endl;

    return 0;
}