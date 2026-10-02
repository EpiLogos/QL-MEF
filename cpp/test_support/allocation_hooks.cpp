#include "allocation_hooks.hpp"
#include <cstdlib>
void *ql_test_allocate(std::size_t bytes) noexcept {
  return std::malloc(bytes ? bytes : 1);
}
void *ql_test_allocate_aligned(std::size_t bytes,
                               std::size_t alignment) noexcept {
  void *allocation = nullptr;
  if (posix_memalign(&allocation, alignment, bytes ? bytes : 1) != 0)
    return nullptr;
  return allocation;
}
void ql_test_release(void *allocation) noexcept { std::free(allocation); }
