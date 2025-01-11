#include <iostream>
int main() {
    int x = 5;
    int& y = x; // mutable reference
    y += 1;
    std::cout << x << std::endl; // prints 6

    std::cout << &x << std::endl; // prints the address of x
    std::cout << &y << std::endl; // prints the address of x
}