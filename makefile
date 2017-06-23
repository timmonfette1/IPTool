# IP Tool Makefile
#
# Makefile for building IP Tool UI
# and IP Tool Script
#
# Author:
# Tim Monfette

# Default builds the script
default: build-script

build-script:
	@if test -d ./src; then echo "src/ directory already exists! Please run 'make clean' before trying to build another project"; else mv iptool-script/ src/; fi
	@cargo build --release

build-ui:
	@if test -d ./src; then echo "src/ directory already exists! Please run 'make clean' before trying to build another project"; else mv iptool-ui/ src/; fi
	@cargo build --release

# clean up back to initial setup
# (no target/, no src/)
.PHONY: clean

clean:
	@rm -rf ./target
	@rm -f ./Cargo.lock
	@if test -d ./src; then if test -d ./iptool-ui; then mv src/ iptool-script/; else mv src/ iptool-ui/; fi; else echo "no src/ directory to clean up"; fi
