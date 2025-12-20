#pragma once
#ifndef __PARSER_HPP__
#define __PARSER_HPP__

#include <stddef.h>
#include "./container.hpp"
#include <string>

namespace Parse {

enum Status {
  OK,
  ERR
};

struct Error {
  std::string error_message = "Some error occured";

  Error() {}
  Error(std::string &&msg): error_message(std::move(msg)) {}

  ~Error() {
    error_message.~basic_string();
  }
};

template <typename T, typename E>
union Either {
  T value;
  E err;

  Either(T &&value): value(std::move(value)) {}

  Either(E &&value): err(std::move(value)) {}

  ~Either() {}
};

template <typename T, typename E = Error>
struct Result {
  Status stat;
  Either<T, E> val;

  Result(T &&value): stat(OK), val(std::move(value)) {}

  Result(E &&value): stat(ERR), val(std::move(value)) {}

  T get() {
    return val.value;
  }

  ~Result() {
    if (stat == OK) {
      val.value.~T();
    } else {
      val.err.~E();
    }
  }

  bool ok() {
    return stat == OK;
  }
};

struct Parser {
  char *ptr;
  size_t current_position;
  size_t length = 0;
  char delimiter = ',';

  struct ParserOptions {
    char delim = ',';
  };

  ParserOptions current_options;

  Parser(const std::string &path) {}

  Parser(ParserOptions&& e);

  char next() {
    return ptr[current_position++];
  }

  bool has_next() {
    return current_position < length;
  }

  char* get_pointer_to_current() {
    return ptr + current_position;
  }

  char get_current_character() {
    return ptr[current_position];
  }

  void read_next();
  Result<Container> read_number();
  // Result<Container> read_into_string_view();
};

enum States {
  STRING,
  INTEGER,
  FLOAT
};

} // namespace Parse

#endif // __PARSER_HPP__
