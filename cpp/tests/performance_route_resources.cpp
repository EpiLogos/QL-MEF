#include <cassert>
#include <iostream>
#include <ql/performance_management.hpp>
#include <sys/resource.h>
using namespace ql::performance;
int main() {
  rlimit limit{};
  assert(getrlimit(RLIMIT_STACK, &limit) == 0);
  // Existing native control owns these large aggregates on the heap. No
  // enlarged thread/OS stack is requested to make continuation work.
  static_assert(sizeof(Engine) < 16 * 1024 * 1024);
  static_assert(sizeof(Engine::Checkpoint) < 8 * 1024 * 1024);
  std::cout << "{\"schema\":\"ql.native-route-resources/v1\",\"Engine\":"
            << sizeof(Engine) << ",\"Operation\":" << sizeof(Operation)
            << ",\"Readback\":" << sizeof(Readback)
            << ",\"Capture\":" << sizeof(Capture)
            << ",\"EngineCheckpoint\":" << sizeof(Engine::Checkpoint)
            << ",\"PairedCheckpoint\":" << sizeof(PairedCheckpoint)
            << ",\"ManagementCheckpoint\":" << sizeof(ManagementCheckpoint)
            << ",\"NativeRouteProgramSet\":" << sizeof(NativeRouteProgramSet)
            << ",\"ordinary_stack_bytes\":" << limit.rlim_cur << "}\n";
}
