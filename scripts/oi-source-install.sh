#!/bin/sh
# O:I managed source install for Quaternal Logic.
#
# Builds the `ql` command and the native Expression owners it relies on at the
# same committed cut: the Rust hosts (ql-field-host, ql-focused-host) and the
# C++ continuous worker (ql-field-worker). O:I stages all of them beside `ql`
# in one content-addressed product directory, so a host can never pair with a
# worker from another revision.
#
# Honours CARGO_TARGET_DIR (the O:I persistent build cache); every executable
# lands in "$CARGO_TARGET_DIR/release".
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
target=${CARGO_TARGET_DIR:-$root/target}
release="$target/release"

cargo build --manifest-path "$root/Cargo.toml" --workspace --locked --release

if [ "$(uname -s)" = Darwin ]; then
  arch=$(uname -m)
  # Link json-c for the architecture actually being built.
  if [ "$arch" = arm64 ] && [ -d /opt/homebrew/opt/json-c/lib/pkgconfig ]; then
    PKG_CONFIG_PATH=/opt/homebrew/opt/json-c/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}
    export PKG_CONFIG_PATH
  fi
  # A Command Line Tools install whose libc++ headers did not land cannot
  # compile <cassert>. Use the SDK's complete libc++ headers instead, and say so.
  probe=$(mktemp -d)
  printf '#include <cassert>\nint main(){}\n' > "$probe/p.cpp"
  if ! c++ -std=c++17 -fsyntax-only "$probe/p.cpp" 2>/dev/null; then
    for sdk in $(ls -d /Library/Developer/CommandLineTools/SDKs/MacOSX*.sdk 2>/dev/null | sort -rV); do
      if [ -f "$sdk/usr/include/c++/v1/cassert" ]; then
        CPLUS_INCLUDE_PATH="$sdk/usr/include/c++/v1"
        export CPLUS_INCLUDE_PATH
        echo "oi-source-install: toolchain libc++ headers missing; using $sdk" >&2
        break
      fi
    done
  fi
  rm -rf "$probe"
fi

build=$(mktemp -d)
trap 'rm -rf "$build"' EXIT
make -C "$root/c" all BUILD_DIR="$build/c"
make -C "$root/cpp" test BUILD_DIR="$build/cpp" LIB="$build/c/libql-mef-c.a"
mkdir -p "$release"
install -m755 "$build/cpp/ql-field-worker" "$release/ql-field-worker"
