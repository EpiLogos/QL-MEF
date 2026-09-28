# Sourced, not executed: the frozen Epi C reference's library set.
#
# The reference kernel is no longer one translation unit: kernel.c calls the
# quaternion operators defined in m1.c, which reach the Ananda tables, and m4
# hashes through the prototype's portable BLAKE3 subset. This is the
# prototype's own LIB_SRC (its root Makefile), read from the vendored tree.
# Callers set REPO_ROOT before sourcing.
EPI_C_REFERENCE_ROOT="$REPO_ROOT/vendor/epi-kernel/reference"
EPI_C_BLAKE3_ROOT="$REPO_ROOT/vendor/epi-kernel/blake3"
EPI_C_REFERENCE_SOURCES=()
for epi_c_unit in psychoid_numbers engine arena families pointer_web \
                  m0 m1 m2 m3 m3_clock_lut m4 m5 kernel; do
  EPI_C_REFERENCE_SOURCES+=("$EPI_C_REFERENCE_ROOT/src/$epi_c_unit.c")
done
EPI_C_REFERENCE_SOURCES+=("$EPI_C_BLAKE3_ROOT/blake3.c" "$EPI_C_BLAKE3_ROOT/blake3_dispatch.c" "$EPI_C_BLAKE3_ROOT/blake3_portable.c")
# The vendored BLAKE3 is the portable C subset only; SIMD backends are off on
# every architecture, including arm64/NEON.
EPI_C_REFERENCE_FLAGS=(-DBLAKE3_NO_SSE2 -DBLAKE3_NO_SSE41 -DBLAKE3_NO_AVX2 -DBLAKE3_NO_AVX512 -DBLAKE3_USE_NEON=0 -I"$EPI_C_REFERENCE_ROOT/include" -I"$EPI_C_BLAKE3_ROOT")
