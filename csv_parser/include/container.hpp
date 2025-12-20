#pragma once

#ifndef __CONTAINER_HPP__
#define __CONTAINER_HPP__

#include <string>
#include <string_view>
#include <ostream>
#include <iostream>
#include <memory>

enum ContainerType {
  STRING,
  STRING_VIEW,
  INTEGER,
  FLOAT,
  END
};

union Wrapper {
  std::string str;
  std::string_view view;
  long long integer;
  double real;

  Wrapper() {}

  Wrapper(std::string &&str): str(std::move(str)) {}
  
  Wrapper(std::string_view str): view(str) {}

  Wrapper(int64_t val): integer(val) {}

  Wrapper(double real): real(real) {}

  ~Wrapper() {} 
};

struct Container {
  ContainerType type;
  Wrapper wrap;

  Container() {}

  Container(std::string&& str): type(STRING), wrap(std::move(str)) {}

  Container(std::string_view str): type(STRING), wrap(str) {}

  Container(int64_t v): type(INTEGER), wrap(v) {}

  Container(double v): type(FLOAT), wrap(v) {}

  Container(Container &&c) {
    switch (c.type) {
    case ContainerType::STRING:
      new (&wrap.str) std::string(std::move(c.wrap.str));
      c.wrap.str.~basic_string();
      break;

    case ContainerType::INTEGER:
      new (&wrap.integer) int(c.wrap.integer);
      break;

    case ContainerType::FLOAT:
      new (&wrap.real) int(c.wrap.real);
      break;

    case ContainerType::STRING_VIEW:
      new (&wrap.view) std::string_view(c.wrap.view);
      break;
    }
  }

  ~Container() {
    switch (type) {
    case ContainerType::STRING:
      wrap.str.~basic_string();
      break;
    }
  }

  friend std::ostream& operator<<(std::ostream& out, const Container& c) {
    switch (c.type) {
    case ContainerType::STRING:
      out << c.wrap.str;
      break;

    case ContainerType::STRING_VIEW:
      out << c.wrap.view;
      break;
    
    case ContainerType::INTEGER:
      out << c.wrap.integer;
      break;
    
    case ContainerType::FLOAT:
      out << c.wrap.real;
      break;
    }

    return out;
  }
};

#endif // __CONTAINER_HPP__