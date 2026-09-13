CXX ?= c++
CXXFLAGS ?= -O2 -g -std=c++17 -Wall -Wextra -Werror -pedantic
PREFIX ?= /usr/local
BUILD_DIR ?= build
C_ROOT ?= ../c
CPPFLAGS += -Iinclude -I$(C_ROOT)/include
JSON_CFLAGS := $(shell pkg-config --cflags json-c)
JSON_LIBS := $(shell pkg-config --libs json-c)
LIB := $(C_ROOT)/build/libql-mef-c.a
.PHONY: all test install
all: $(BUILD_DIR)/ql-field-worker $(BUILD_DIR)/continuous-test
$(BUILD_DIR):
	mkdir -p $@
$(LIB):
	$(MAKE) -C $(C_ROOT) all
$(BUILD_DIR)/ql-field-worker: src/field_worker.cpp include/ql/continuous_field.hpp $(LIB) | $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(JSON_CFLAGS) $< $(LIB) $(JSON_LIBS) -lm -o $@
$(BUILD_DIR)/continuous-test: tests/continuous_field.cpp include/ql/continuous_field.hpp $(LIB) | $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) $(CPPFLAGS) $< $(LIB) -lm -o $@
test: all
	$(BUILD_DIR)/continuous-test
install: all
	$(MAKE) -C $(C_ROOT) install PREFIX=$(PREFIX) DESTDIR=$(DESTDIR)
	install -d $(DESTDIR)$(PREFIX)/include/ql $(DESTDIR)$(PREFIX)/bin
	install -m644 include/ql/continuous_field.hpp $(DESTDIR)$(PREFIX)/include/ql/
	install -m755 $(BUILD_DIR)/ql-field-worker $(DESTDIR)$(PREFIX)/bin/
