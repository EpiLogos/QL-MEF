CXX ?= c++
CXXFLAGS ?= -O2 -g -std=c++17 -Wall -Wextra -Werror -pedantic
PREFIX ?= /usr/local
BUILD_DIR ?= build
C_ROOT ?= ../c
CPPFLAGS += -Iinclude -I$(C_ROOT)/include
JSON_CFLAGS := $(shell pkg-config --cflags json-c | sed 's/-I/-isystem /g')
JSON_LIBS := $(shell pkg-config --libs json-c)
# On Darwin, Homebrew LLVM/json-c need runtime search paths when CXX is not the CLT.
comma := ,
RPATH_FLAGS :=
ifeq ($(shell uname -s),Darwin)
  # Apple clang searches /usr/local/include ahead of every -isystem path, so on a
  # Mac carrying both Intel and arm64 Homebrew the wrong-architecture json-c
  # headers win. Take pkg-config's json-c as an ordinary -I (searched first) and
  # keep its headers out of -Werror as system headers.
  JSON_CFLAGS := $(shell pkg-config --cflags json-c) --system-header-prefix=json-c/
  RPATH_FLAGS += $(foreach d,$(wildcard /opt/homebrew/opt/llvm/lib/c++ /opt/homebrew/opt/llvm/lib/unwind /opt/homebrew/opt/json-c/lib /usr/local/opt/llvm/lib/c++ /usr/local/opt/json-c/lib),-Wl$(comma)-rpath$(comma)$(d))
endif

LIB := $(C_ROOT)/build/libql-mef-c.a
HEADERS := $(wildcard include/ql/*.hpp)
TEST_SOURCES := $(wildcard tests/*.cpp)
# Stdin transport probes are exercised by their actual Rust producers below
# the native verification command; every other native suite runs directly.
TRANSPORT_SOURCES := $(filter %_wire.cpp %_packet.cpp,$(TEST_SOURCES))
# Installed probes need the installation/qualification supplied by their
# existing owner campaigns; discovering a source does not invent those inputs.
INSTALLED_SOURCES := $(filter %_installed.cpp,$(TEST_SOURCES))
UNIT_SOURCES := $(filter-out tests/continuous_field.cpp $(TRANSPORT_SOURCES) $(INSTALLED_SOURCES),$(TEST_SOURCES))
UNIT_TESTS := $(patsubst tests/%.cpp,$(BUILD_DIR)/%-test,$(UNIT_SOURCES))
TRANSPORT_TESTS := $(patsubst tests/%.cpp,$(BUILD_DIR)/%-test,$(TRANSPORT_SOURCES))
ALLOCATION_HOOKS := test_support/allocation_hooks.cpp test_support/allocation_hooks.hpp
MAC_DEVICE_OBJECT :=
ifeq ($(shell uname -s),Darwin)
  MAC_DEVICE_OBJECT := $(BUILD_DIR)/audio-device-macos.o
endif
.PHONY: all test install
all: $(BUILD_DIR)/ql-field-worker $(BUILD_DIR)/continuous-test $(UNIT_TESTS) $(TRANSPORT_TESTS) $(MAC_DEVICE_OBJECT)
$(BUILD_DIR):
	mkdir -p $@
$(LIB):
	$(MAKE) -C $(C_ROOT) all
$(BUILD_DIR)/ql-field-worker: src/field_worker.cpp include/ql/continuous_field.hpp $(LIB) | $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(JSON_CFLAGS) $< $(LIB) $(JSON_LIBS) $(RPATH_FLAGS) -lm -o $@
$(BUILD_DIR)/continuous-test: tests/continuous_field.cpp include/ql/continuous_field.hpp $(LIB) | $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) $(CPPFLAGS) $< $(LIB) -lm -o $@
$(BUILD_DIR)/%-test: tests/%.cpp $(HEADERS) $(ALLOCATION_HOOKS) $(LIB) | $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(JSON_CFLAGS) $< test_support/allocation_hooks.cpp $(LIB) $(JSON_LIBS) $(RPATH_FLAGS) -lm -o $@
$(BUILD_DIR)/audio-device-macos.o: src/audio_device_macos.cpp $(HEADERS) | $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) $(CPPFLAGS) -c $< -o $@
test: all
	$(BUILD_DIR)/continuous-test
	@set -e; for suite in $(UNIT_TESTS); do "$$suite"; done
install: all
	$(MAKE) -C $(C_ROOT) install PREFIX=$(PREFIX) DESTDIR=$(DESTDIR)
	install -d $(DESTDIR)$(PREFIX)/include/ql $(DESTDIR)$(PREFIX)/bin
	install -m644 $(HEADERS) $(DESTDIR)$(PREFIX)/include/ql/
	install -m755 $(BUILD_DIR)/ql-field-worker $(DESTDIR)$(PREFIX)/bin/

	install -d $(DESTDIR)$(PREFIX)/share/ql-mef-c/adapters/retained-field
	install -m644 ../adapters/retained-field/retained-field.mjs ../adapters/retained-field/native-audio.mjs ../adapters/retained-field/instrument-session.mjs $(DESTDIR)$(PREFIX)/share/ql-mef-c/adapters/retained-field/
