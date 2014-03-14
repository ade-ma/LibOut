CC=rustc

ifeq ($(ARCH),arm)
CFLAGS=-O -L ./lib -A unused-variable -A unused-imports --target arm-unknown-linux-gnueabihf -C linker=arm-linux-gnueabihf-gcc -C link-args=-Wl,-rpath-link,$(PWD)/lib
else
CFLAGS=-O -L ./lib -A unused-variable -A unused-imports
endif

OBJ = ./lib/liblibusb*.rlib ./lib/libusb*.rlib ./lib/liboblw*.rlib ./lib/libtoml*.rlib

all: $(OBJ)
	mkdir -p bin
	$(CC) $(CFLAGS) ./src/outlet.rs
	mv outlet bin

./lib/lib%.rlib: ./src/%.rs
	mkdir -p lib
	$(CC) $(CFLAGS) --crate-type=lib $<
	mv -f *rlib lib

clean:
	rm -r lib/*rlib bin
