#ifndef QL_TEST_ALLOCATION_HOOKS_HPP
#define QL_TEST_ALLOCATION_HOOKS_HPP
#include <cstddef>
// The replacement C++ allocation operators in native callback tests count real
// allocations and frees. Keep their C allocation implementation in a separate
// translation unit so the compiler preserves the replacement-operator boundary.
void *ql_test_allocate(std::size_t bytes) noexcept;
void *ql_test_allocate_aligned(std::size_t bytes,
                               std::size_t alignment) noexcept;
void ql_test_release(void *allocation) noexcept;
#endif
