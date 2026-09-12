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
CONTRACT_VERSION = 1.1.0
PACKAGE_BASENAME = ql-mef-c-$(API_VERSION)-$(SOURCE_REVISION).tar.gz
LIB = $(BUILD_DIR)/libql-mef-c.a
SRC = src/primitive.c src/holographic.c src/kernel.c src/m_tree.c src/m_tree_live.c src/coupled_clock.c src/m2_aperture.c src/m1_state.c src/m1.c src/m2.c src/m3.c src/m3_domain.c
OBJ = $(SRC:src/%.c=$(BUILD_DIR)/%.o)
HEADERS = include/ql/primitive.h include/ql/holographic.h include/ql/kernel.h include/ql/m_tree.h include/ql/m_tree_live.h include/ql/coupled_clock.h include/ql/m2_aperture.h include/ql/m1.h include/ql/m2.h include/ql/m3.h include/ql/m3_domain.h
REVISION_STAMP = $(BUILD_DIR)/source-revision.txt
M_TREE_MANIFEST = ../fixtures/kernel/m-tree-v1.json

CONTRACT_FIXTURE = ../fixtures/kernel/holographic-kernel-contract-v1.tsv

.PHONY: all clean install package FORCE

all: $(LIB)

$(BUILD_DIR):
	mkdir -p $@

# A new checkout/revision must rebuild the macro embedded in the library even
# when source mtimes are unchanged. Keep the stamp mtime on a no-op invocation.
FORCE:

$(REVISION_STAMP): FORCE | $(BUILD_DIR)
	@printf '%s\n' '$(SOURCE_REVISION)' > $@.tmp
	@cmp -s $@.tmp $@ || cp $@.tmp $@
	@rm -f $@.tmp

$(BUILD_DIR)/%.o: src/%.c $(HEADERS) $(REVISION_STAMP) | $(BUILD_DIR)
	$(CC) $(CPPFLAGS) $(CFLAGS) -Iinclude -DQL_C_SOURCE_REVISION='"$(SOURCE_REVISION)"' -c $< -o $@

$(BUILD_DIR)/m_tree.o: src/m_tree_data.inc

$(BUILD_DIR)/live/m_tree_live_data.inc: ../scripts/k8-structure.py ../scripts/generate-m-tree.py registry/m-tree-source-v1.json ../c/registry/promotions/k8-apertures-v1.json ../fixtures/kernel/m-tree-v1.json ../fixtures/kernel/k8-structure-receipt-v1.json ../docs/kernel-rebuild/APERTURES-AND-CLOCK-CENTRE.md | $(BUILD_DIR)
	python3 ../scripts/k8-structure.py --check --out $(BUILD_DIR)/live

$(BUILD_DIR)/live/m_tree_live_origins.inc $(BUILD_DIR)/live/m2_aperture_data.inc $(BUILD_DIR)/live/m-tree-v2.json $(BUILD_DIR)/live/k8-structure-receipt-v1.json: $(BUILD_DIR)/live/m_tree_live_data.inc
	@test -f $@ || python3 ../scripts/k8-structure.py --check --out $(BUILD_DIR)/live

$(BUILD_DIR)/m_tree_live.o: $(BUILD_DIR)/live/m_tree_live_data.inc $(BUILD_DIR)/live/m_tree_live_origins.inc src/m_tree.c
$(BUILD_DIR)/m_tree_live.o: CPPFLAGS += -I$(BUILD_DIR)/live
$(BUILD_DIR)/m2_aperture.o: $(BUILD_DIR)/live/m2_aperture_data.inc
$(BUILD_DIR)/m2_aperture.o: CPPFLAGS += -I$(BUILD_DIR)/live
$(BUILD_DIR)/m1_state.o: src/m1_source_data.inc
$(BUILD_DIR)/m2.o: src/m2_data.inc src/m2_correspondence_data.inc

$(BUILD_DIR)/m3_data.inc: ../scripts/generate-m3.py ../fixtures/kernel/m-tree-v1.json ../vendor/epi-kernel/reference/src/m3.c ../vendor/epi-kernel/reference/src/m3_clock_lut.c ../vendor/epi-kernel/reference/include/m3.h | $(BUILD_DIR)
	python3 ../scripts/generate-m3.py --out $@

$(BUILD_DIR)/m3.o: $(BUILD_DIR)/m3_data.inc
$(BUILD_DIR)/m3.o: CPPFLAGS += -I$(BUILD_DIR)

$(BUILD_DIR)/m3_domain_data.inc: ../scripts/m3-domain.py ../scripts/m3-source-parity.py ../fixtures/kernel/m3-domain-v1.json ../fixtures/kernel/m-tree-v1.json | $(BUILD_DIR)
	python3 ../scripts/m3-domain.py --c-output $@

$(BUILD_DIR)/m3_domain.o: $(BUILD_DIR)/m3_domain_data.inc
$(BUILD_DIR)/m3_domain.o: CPPFLAGS += -I$(BUILD_DIR)

$(LIB): $(OBJ)
	$(AR) $(ARFLAGS) $@ $^

install: $(LIB) $(BUILD_DIR)/live/m-tree-v2.json $(BUILD_DIR)/live/k8-structure-receipt-v1.json
	test -f $(CONTRACT_FIXTURE)
	mkdir -p $(DESTDIR)$(PREFIX)/include/ql $(DESTDIR)$(PREFIX)/lib $(DESTDIR)$(PREFIX)/share/ql-mef-c
	cp $(HEADERS) $(DESTDIR)$(PREFIX)/include/ql/
	cp $(LIB) $(DESTDIR)$(PREFIX)/lib/libql-mef-c.a
	cp $(M_TREE_MANIFEST) $(DESTDIR)$(PREFIX)/share/ql-mef-c/m-tree-v1.json
	cp $(BUILD_DIR)/live/m-tree-v2.json $(DESTDIR)$(PREFIX)/share/ql-mef-c/m-tree-v2.json
	cp $(BUILD_DIR)/live/k8-structure-receipt-v1.json $(DESTDIR)$(PREFIX)/share/ql-mef-c/k8-structure-receipt-v1.json
	cp $(CONTRACT_FIXTURE) $(DESTDIR)$(PREFIX)/share/ql-mef-c/holographic-kernel-contract-v1.tsv
	printf '%s\n' '$(API_VERSION)' > $(DESTDIR)$(PREFIX)/share/ql-mef-c/api-version.txt
	printf '%s\n' '$(CONTRACT_VERSION)' > $(DESTDIR)$(PREFIX)/share/ql-mef-c/kernel-contract-version.txt
	cp $(REVISION_STAMP) $(DESTDIR)$(PREFIX)/share/ql-mef-c/source-revision.txt

package: $(LIB)
	rm -rf $(BUILD_DIR)/package-root
	$(MAKE) install DESTDIR=$(abspath $(BUILD_DIR)/package-root) PREFIX=/ql-mef-c SOURCE_REVISION='$(SOURCE_REVISION)'
	mkdir -p $(DIST_DIR)
	tar --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner -C $(BUILD_DIR)/package-root -cf - ql-mef-c | gzip -n > $(DIST_DIR)/$(PACKAGE_BASENAME)
	cd $(DIST_DIR) && sha256sum $(PACKAGE_BASENAME) > $(PACKAGE_BASENAME).sha256

clean:
	rm -rf $(BUILD_DIR) $(DIST_DIR)
