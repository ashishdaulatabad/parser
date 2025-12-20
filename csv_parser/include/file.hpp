#pragma once

#ifndef __FILE_HPP__
#define __FILE_HPP__
#ifdef _WIN32
#include <Windows.h>
#endif

#include <stdio.h>

#ifdef __linux__
#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>
#endif

struct FileReader {
  char *buffer;
  size_t size;
  size_t pointer = 0;
  FILE *iptr;
#ifdef _WIN32
  HANDLE file;
  void *handle;
#endif

#ifdef __linux__
  int file_descriptor;
#endif

  /**
   * @brief Get size of the file
   * @param filename File to check
   * @returns size of the file (long long unsigned)
   */
  size_t get_file_size(const char *filename) const {
    FILE *p_file = fopen(filename, "rb");
    fseek(p_file, 0, SEEK_END);
    size_t size = ftello64(p_file);
    fclose(p_file);
    return size;
  }

  /**
   * @brief read next character from given buffer
   */
  inline char read_next_char() {
    return (pointer >= size ? EOF : buffer[pointer++]);
  }

  /**
   * @brief Read next character and move to next value
   * but user has control over the pointer.
   * @param __offset current offset
   * @returns the character at offset `__offset`
   */
  inline char read_char_offset(size_t &__offset) const {
    return (__offset >= size ? EOF : buffer[__offset++]);
  }

  /**
   * @brief Close the file, and unmap the file
   * @return void
   */
  inline void close() {
#ifdef _WIN32
      UnmapViewOfFile(buffer);
      CloseHandle(file);
        // fclose(iptr);
#endif
#ifdef __linux__
      munmap((void *)buffer, size);
      ::close(file_descriptor);
#endif
    }

    /**
     * @brief Loads the file using memory mapping
     * @param filepath path to the file to be opened
     * @returns void
     */
  inline void load_file(const char *filepath) {
    size = get_file_size(filepath);

#ifdef _WIN64
      // Open file for this program
      // FILE_SHARE_READ: to have only shared read access and not write.
      // OPEN_EXISTING: only open when it exists, returns error if file does
      // not exists FILE_ATTRIBUTE_READONLY: read only flag
      // FILE_SEQUENCIAL_SCAN: optimize for sequential scanning
    file = CreateFile(filepath, GENERIC_READ, FILE_SHARE_READ, NULL,
                      OPEN_EXISTING, FILE_ATTRIBUTE_READONLY, NULL);
    OVERLAPPED ol = {0};
    handle = CreateFileMapping(file, NULL, PAGE_READONLY, 0, 0, "");
    buffer = (char *)MapViewOfFile(handle, FILE_MAP_READ, 0, 0, 0);
#endif

#ifdef __linux__
      // std::cout << size << std::endl;
    file_descriptor = open(filepath, O_RDONLY);
    buffer = (char *)mmap(0, size, PROT_READ, MAP_SHARED, file_descriptor, 0);
    if (buffer == MAP_FAILED) {
      perror("Map Failed");
    }
#endif
  }
};

#endif
