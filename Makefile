CC=rustc
CFLAGS=-O -L ./lib -A unused-variable -A unused-imports

OBJ = ./lib/liblibusb*.rlib ./lib/libusb*.rlib ./lib/liboblw*.rlib 

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
