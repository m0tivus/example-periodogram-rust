ECC=emcc
CFLAGS=-O3
datadir=@datadir@

SOURCE_DIR ?= spqr/src
BUILD_DIR ?= build
objdir=obj
FILESYSTEM_DIR ?= filesystem
LOG_STDOUT_STDERR ?= false
wbadapter=waterbear.wasmpack.js

PACKAGE_NAME ?= Test
PACKAGE_VERSION ?= 1.0.0
PACKAGE=$(PACKAGE_NAME)-$(PACKAGE_VERSION)

# BEGIN app specific
MMCDEPS=mpi_mc_mi.o mc_checkpoints.o  mc_end_mi.o  mc_energies_mi.o  mc_initialization_mi.o mc_integrate_mi.o  mc_verlet_lists.o  mc_global.o mc_base_pairing.o  mc_utils.o mc_ermsd_mi.o
PREFIXED_OBJ_FILES=$(addprefix $(objdir)/, $(MMCDEPS))


$(objdir)/%.o : $(SOURCE_DIR)/%.c
	$(ECC) $(CFLAGS) -DFROZEN -o $@ -c $<
$(objdir)/%_mi.o : $(SOURCE_DIR)/%.c
	$(ECC) $(CFLAGS) -DFROZEN -DNOCTCS -DERMSDR  -o $@ -c $<

# This is SPQR_mMC compilation description extracted from spqr/src/Makefile.in
# The compiler backend, different output file and additional emscripten flags are defined
app:
	wasm-pack build --target no-modules --verbose --out-name $(PACKAGE) --out-dir $(BUILD_DIR)
	#$(PREFIXED_OBJ_FILES)
	#$(ECC) $(CFLAGS) $(PREFIXED_OBJ_FILES) -DFROZEN -DSPQR_DATA=$(datadir)/spqr -o $(BUILD_DIR)/$(PACKAGE).js -lm --preload-file $(FILESYSTEM_DIR)@/ -s ASSERTIONS=1 -s ALLOW_MEMORY_GROWTH=1 -s EXPORTED_RUNTIME_METHODS=FS,PATH 
# END app specific

# BEGIN packager common
dirs:
	[ -d $(BUILD_DIR) ] || mkdir -p $(BUILD_DIR)
	[ -d $(objdir) ] || mkdir -p $(objdir)

# Take emscripten output files and create motivus compatible package
package:
	cat $(wbadapter) >> $(BUILD_DIR)/$(PACKAGE).js
#	mv $(BUILD_DIR)/$(PACKAGE).js $(BUILD_DIR)/emscripten.$(PACKAGE).js
#	# Add motivus adapter
#	echo } | cat $(wbadapter) $(BUILD_DIR)/emscripten.$(PACKAGE).js - > $(BUILD_DIR)/worker.$(PACKAGE).js
#	if [ $(LOG_STDOUT_STDERR) = true ]; then (echo "Module.print = console.log; Module.printErr = console.log;" >> $(BUILD_DIR)/worker.$(PACKAGE).js) fi
#	rm -f $(BUILD_DIR)/emscripten.$(PACKAGE).js
#	npm install
#	# Make js web compatible
#	webpack --entry ./$(BUILD_DIR)/worker.$(PACKAGE).js -o ./$(BUILD_DIR) --config webpack.config.js
#	rm -f $(BUILD_DIR)/worker.$(PACKAGE).js
#	cd $(BUILD_DIR) && mv main.js $(PACKAGE).js
#	cd $(BUILD_DIR) && zip $(PACKAGE).data.zip -m $(PACKAGE).data
#	# Zip package contents for publishing
#	cd $(BUILD_DIR) && zip $(PACKAGE).zip $(PACKAGE).* -x $(PACKAGE).zip

clean:
	rm -rf $(BUILD_DIR) $(objdir)
# END packager common

# required
dist: dirs app package

