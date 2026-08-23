CC ?= cc
AR ?= ar
ARFLAGS = rcsD
CFLAGS ?= -std=c11 -O2 -Wall -Wextra -Werror -pedantic
CPPFLAGS ?=
PREFIX ?= /usr/local
BUILD_DIR ?= build
DIST_DIR ?= dist
SOURCE_REVISION ?= $(shell git rev-parse HEAD 2>/dev/null || printf unversioned)

API_VERSION = 0.1.0
CONTRACT_VERSION = 1.0.0
PACKAGE_BASENAME = ql-mef-c-$(API_VERSION)-$(SOURCE_REVISION).tar.gz
LIB = $(BUILD_DIR)/libql-mef-c.a
SRC = src/primitive.c src/holographic.c src/kernel.c
OBJ = $(SRC:src/%.c=$(BUILD_DIR)/%.o)
HEADERS = include/ql/primitive.h include/ql/holographic.h include/ql/kernel.h
CONTRACT_FIXTURE = ../fixtures/kernel/holographic-kernel-contract-v1.tsv

.PHONY: all clean install package

all: $(LIB)

$(BUILD_DIR):
	mkdir -p $@

$(BUILD_DIR)/%.o: src/%.c $(HEADERS) | $(BUILD_DIR)
	$(CC) $(CPPFLAGS) $(CFLAGS) -Iinclude -DQL_C_SOURCE_REVISION='"$(SOURCE_REVISION)"' -c $< -o $@

$(LIB): $(OBJ)
	$(AR) $(ARFLAGS) $@ $^

install: $(LIB)
	test -f $(CONTRACT_FIXTURE)
	mkdir -p $(DESTDIR)$(PREFIX)/include/ql $(DESTDIR)$(PREFIX)/lib $(DESTDIR)$(PREFIX)/share/ql-mef-c
	cp $(HEADERS) $(DESTDIR)$(PREFIX)/include/ql/
	cp $(LIB) $(DESTDIR)$(PREFIX)/lib/libql-mef-c.a
	cp $(CONTRACT_FIXTURE) $(DESTDIR)$(PREFIX)/share/ql-mef-c/holographic-kernel-contract-v1.tsv
	printf '%s\n' '$(API_VERSION)' > $(DESTDIR)$(PREFIX)/share/ql-mef-c/api-version.txt
	printf '%s\n' '$(CONTRACT_VERSION)' > $(DESTDIR)$(PREFIX)/share/ql-mef-c/kernel-contract-version.txt
	printf '%s\n' '$(SOURCE_REVISION)' > $(DESTDIR)$(PREFIX)/share/ql-mef-c/source-revision.txt

package: $(LIB)
	rm -rf $(BUILD_DIR)/package-root
	$(MAKE) install DESTDIR=$(abspath $(BUILD_DIR)/package-root) PREFIX=/ql-mef-c SOURCE_REVISION='$(SOURCE_REVISION)'
	mkdir -p $(DIST_DIR)
	tar --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner -C $(BUILD_DIR)/package-root -cf - ql-mef-c | gzip -n > $(DIST_DIR)/$(PACKAGE_BASENAME)
	cd $(DIST_DIR) && sha256sum $(PACKAGE_BASENAME) > $(PACKAGE_BASENAME).sha256

clean:
	rm -rf $(BUILD_DIR) $(DIST_DIR)
