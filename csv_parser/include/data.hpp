#pragma once

#ifndef __DATA_HPP__
#define __DATA_HPP__

#include <vector>
#include <string>
#include <string_view>
#include <stdexcept>

// Should contain all the data.
struct DataView {
  std::vector<std::string> header;
  std::vector<std::vector<std::string_view>> column_data_view;
  size_t size;

  DataView(): size(0) {}

  void reserve_data(size_t size) {
    for (auto &column: column_data_view) {
      column.reserve(size);
    }
    this->size = size;
  }

  void set_total_columns(size_t total_columns) {
    header.reserve(total_columns);
    column_data_view.reserve(total_columns);
  }

  void add_to_column(size_t column_index, std::string_view detail) {
    if (column_index >= header.size()) {
      throw std::runtime_error("Column index requested out of index");
    }

    column_data_view[column_index].emplace_back(detail);
  }

  void set_to_column(size_t column_index, size_t cell_index, std::string_view detail) {
    if (column_index >= header.size()) {
      throw std::runtime_error("Column index requested out of index");
    }

    if (cell_index >= column_data_view.size()) {
      throw std::runtime_error("Cell index requested out of index");
    }

    column_data_view[column_index][cell_index] = detail;
  }
};

#endif // __DATA_HPP__