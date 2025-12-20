#include "./parser.hpp"
#include "./container.hpp"

namespace Parse {

Parser::Parser(Parser::ParserOptions&& e): current_options(std::move(e)) {
  delimiter = current_options.delim;
}

void Parser::read_next() {
  while (has_next()) {
    switch (get_current_character()) {
    case '"':
      // Reading a string
      break;

    case '0'...'9':
    case '-':
    case '+':
    case '.':
      // Reading a number
      break;
    }
  }
}

Result<Container> Parser::read_number() {
  const char* starting_position = get_pointer_to_current();
  int64_t value = 0;
  double maybe_float = 0;

  // States
  bool is_negative = false;
  bool is_exponent_negative = false;
  bool is_decimal_read = false;
  bool is_exponent_read = false;
  bool is_floating_point = false;
  bool is_end_of_scan = false;
  bool is_nan = true;
  double precision = 0, den = 10;

  // TODO Instead of returning parse error on numbers, return as string.
  while (has_next() && !is_end_of_scan) {
    char c = next();

    if (is_nan) {
      switch (c) {
      case ',':
        is_end_of_scan = true;
        break;
      }
    } else {
      switch (c) {
      case '0'...'9':
        if (!is_floating_point) {
          value *= 10;
          value += c - '0';
        } else {
          precision += (c - '0') / den;
          den *= 10;
        }

        break;

      case 'e':
      case 'E':
        if (is_exponent_read) {
          is_nan = true;
        } else {
          is_exponent_read = true;
          is_floating_point = true;
          maybe_float += value + precision;
        }
        break;

      case '.':
        if (is_floating_point) {
          is_nan = true;
        } else {
          is_floating_point = true;
        }
        break;      

      case '-':
        if (is_exponent_read) {
          if (is_exponent_negative) {
            is_nan = true;
          } else {
            is_exponent_negative = true;
          }
        } else {
          if (is_negative) {
            is_nan = true;
          } else {
            is_negative = true;
          }
        }
        break;

      case ',':
      case '\n':
      default:
        is_end_of_scan = true;
        break;
      }
    }
  }

  Container result = is_nan ? Container(std::string_view(
      starting_position,
      get_pointer_to_current() - starting_position
    )
  ) : (is_floating_point ? Container(maybe_float) : Container(value));
  
  return result;
}

// TODO: Create either a string view or string.
// Result<Container> Parser::read_into_string_view() {
//   while (has_next()) {
//     char ch = next().get();

//     switch (ch) {
//     case '"':
//       // Check if previous content was "\"
//     }
//   }
// }

} // namespace Parse
