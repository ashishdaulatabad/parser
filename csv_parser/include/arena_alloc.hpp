#pragma once
#ifndef __ARENA_ALLOC_HPP__
#define __ARENA_ALLOC_HPP__

#include <iostream>

struct Arena {
  void *allocated_mem;
  size_t size;
  size_t start = 0, offset = 0;

  // Should I keep track of memory inside the allocated_mem itself?
  Arena(size_t total_size_bytes): size(total_size_bytes) {
    allocated_mem = malloc(size);
  }

  // Get the allocated size (will return `size`)
  void* allocate(const size_t size) {
    // Handle out-of-bound case.
    if (offset + size >= size) {
      return nullptr;
    }

    void* offset_ptr = allocated_mem + offset;
    offset += size;

    return offset_ptr;
  }


  void deallocate(void* ptr) {
    if (allocated_mem + start == ptr) {
      // logically free the memory
      // should I memset?
      
    }
  }

};

#endif //  __ARENA_ALLOC_HPP__
