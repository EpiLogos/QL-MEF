#ifndef QL_WORKER_MALLOC_OBSERVATION_HPP
#define QL_WORKER_MALLOC_OBSERVATION_HPP
// Diagnostic only, compiled solely on Apple. Never called from a callback.
#if defined(__APPLE__)
#include <array>
#include <charconv>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <limits>
#include <malloc/malloc.h>
#include <ql/continuous_field.hpp>
#include <string_view>
#include <unistd.h>
namespace ql::worker_diagnostic {
struct RequestMallocFacts {
  const char *operation = "unparsed", *domain = "unparsed";
  std::size_t request_bytes = 0, serialized_reply_bytes = 0;
  std::size_t requested_field_frames = 0, rendered_field_frames = 0;
  bool exception_reply = false;
};
// No owned dynamic buffer, timer, VM operation or pressure-relief mutation. Request
// ordinals are selected by the guarded custodian, not an inferred clock.
class RequestMallocObserver {
  std::array<std::uint64_t, 3> selected_{};
  unsigned count_ = 0, next_ = 0;
  std::uint64_t completed_ = 0, field_frames_ = 0;
  bool count_overflow_ = false;

public:
  RequestMallocObserver() noexcept {
    const char *input = std::getenv("QL_NATIVE_WORKER_MALLOC_TRACE_REQUESTS");
    if (!input || !*input)
      return;
    const auto length = ::strnlen(input, 64);
    if (!length || length >= 64)
      return;
    const char *end = input + length, *cursor = input;
    unsigned count = 0;
    while (cursor != end) {
      if (count == selected_.size() || *cursor < '1' || *cursor > '9')
        return;
      const char *finish = cursor;
      while (finish != end && *finish != ',')
        ++finish;
      std::uint64_t value = 0;
      const auto parsed = std::from_chars(cursor, finish, value);
      if (parsed.ec != std::errc{} || parsed.ptr != finish || !value ||
          (count && value <= selected_[count - 1]))
        return;
      selected_[count++] = value;
      if (finish == end)
        break;
      cursor = finish + 1;
      if (cursor == end)
        return;
    }
    count_ = count;
  }
  bool enabled() const noexcept { return next_ < count_; }
  bool selects_current_request() const noexcept {
    return enabled() && completed_ < std::numeric_limits<std::uint64_t>::max() &&
           selected_[next_] == completed_ + 1;
  }
  static const char *operation_name(std::string_view input) noexcept {
    constexpr std::string_view names[] = {
        "initialize", "advance", "read", "set-axis", "replace-modes",
        "replace-shapes", "prepare", "catalog", "press", "release",
        "expression", "sustain", "panic", "hold", "parameter",
        "parameter-clear", "parameter-undo", "parameter-learn",
        "device-enumerate", "device-open", "device-start", "device-stop",
        "device-recover", "device-close", "score", "timing", "checkpoint",
        "offline-render", "restore", "restore-current-receiving", "inspect",
        "receiving-transport-install", "receiving-transport-replace"};
    for (const auto name : names)
      if (input == name)
        return name.data();
    return "unknown";
  }
  // Iteration guard invokes this AFTER request, tokener, output, local audio
  // and operation/schema strings are destroyed. Persistent FIELD/basis/input
  // capacity intentionally remain. A performance flag never aliases its cursor.
  void completed(const RequestMallocFacts &facts, const ContinuousField *field,
                 bool performance_active, std::size_t retained_line_capacity,
                 bool state_committed) noexcept {
    if (!enabled())
      return;
    if (completed_ == std::numeric_limits<std::uint64_t>::max()) {
      count_ = next_;
      return;
    }
    ++completed_;
    if (facts.rendered_field_frames >
        std::numeric_limits<std::uint64_t>::max() - field_frames_)
      count_overflow_ = true;
    else
      field_frames_ += facts.rendered_field_frames;
    if (completed_ != selected_[next_])
      return;
    ++next_; // At most three actual statistics calls and stderr records.
    malloc_statistics_t stats{};
    // SDK malloc.h: NULL sums ALL zones; size_allocated is reserved memory,
    // not process charged footprint, RSS or virtual size.
    ::malloc_zone_statistics(nullptr, &stats);
    char fields[512]{};
    if (field) {
      const auto receipt = field->receipt();
      std::size_t shape_vectors = 0;
      for (const auto &sample : field->samples())
        shape_vectors += sample.mode_shapes.size();
      std::snprintf(fields, sizeof(fields),
                    "\"field_sample_count\":%zu,\"field_mode_count\":%zu,"
                    "\"field_shape_vector_count\":%zu,"
                    "\"field_samples_elapsed\":\"%llu\","
                    "\"field_generation\":\"%llu\"",
                    field->samples().size(), field->source().modes.size(),
                    shape_vectors,
                    static_cast<unsigned long long>(receipt.samples_elapsed),
                    static_cast<unsigned long long>(receipt.generation));
    } else {
      std::snprintf(fields, sizeof(fields),
                    "\"field_sample_count\":null,\"field_mode_count\":null,"
                    "\"field_shape_vector_count\":null,"
                    "\"field_samples_elapsed\":null,\"field_generation\":null");
    }
    char output[2048]{};
    const int size = std::snprintf(
        output, sizeof(output),
        "{\"schema\":\"ql.native-worker-malloc-observation/v1\","
        "\"phase\":\"after-request-destruction\",\"pid\":%ld,"
        "\"completed_request_ordinal\":\"%llu\",\"domain\":\"%s\","
        "\"operation\":\"%s\",\"request_bytes\":%zu,"
        "\"serialized_reply_bytes\":%zu,\"exception_reply\":%s,"
        "\"state_committed\":%s,\"requested_field_frames\":%zu,"
        "\"rendered_field_frames\":%zu,\"rendered_field_frames_total\":\"%llu\","
        "\"frame_counter_overflow\":%s,\"retained_line_capacity\":%zu,"
        "\"performance_owner_active\":%s,%s,\"malloc_zone_scope\":\"all\","
        "\"blocks_in_use\":%u,\"size_in_use\":%zu,\"max_size_in_use\":%zu,"
        "\"size_allocated\":%zu}\n",
        static_cast<long>(::getpid()),
        static_cast<unsigned long long>(completed_), facts.domain,
        facts.operation, facts.request_bytes, facts.serialized_reply_bytes,
        facts.exception_reply ? "true" : "false",
        state_committed ? "true" : "false", facts.requested_field_frames,
        facts.rendered_field_frames,
        static_cast<unsigned long long>(field_frames_),
        count_overflow_ ? "true" : "false", retained_line_capacity,
        performance_active ? "true" : "false", fields, stats.blocks_in_use,
        stats.size_in_use, stats.max_size_in_use, stats.size_allocated);
    if (size <= 0 || std::size_t(size) >= sizeof(output))
      return;
    // Single bounded stderr write. Missing/partial evidence fails external
    // qualification; diagnostic I/O never changes stdout or native state.
    // The guarded custodian owns stderr backpressure.
    (void)::write(STDERR_FILENO, output, static_cast<std::size_t>(size));
  }
};
} // namespace ql::worker_diagnostic
#endif
#endif
