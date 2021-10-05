#include <iostream>

#include "bst.hpp"

int main() {
    // build sample tree containing 3, 5
    bst<int> sample{5};
    sample.insert(3);
    // clone
    bst<int> sample2{sample};
    // add 7 to original tree (should not affect clone)
    sample.insert(7);
    
    // print trees
    std::cout << sample << std::endl;
    std::cout << sample2 << std::endl;
    for (auto it = sample.begin(); it != sample.end(); ++it) {
        const auto& val = *it;
        std::cout << val << std::endl;
    }
}