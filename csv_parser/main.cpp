#include <iostream>
#include <string>
#include <vector>
#include "./include/container.hpp"
#include "./include/parser.hpp"

int main () {
  std::vector<Container> vec;
  vec.emplace_back(std::string("Hello"));
  vec.emplace_back(std::string("12"));
  vec.emplace_back(12.34);
  vec.emplace_back(static_cast<int64_t>(1986666));

  for (const Container &x: vec) {
    std::cout << x << " (" << x.type << "), ";
  }

  std::cout << std::endl;
}
