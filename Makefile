CC = gcc
CFLAGS = -O3 -march=native -std=c99 -pedantic -Wall -Wextra -Wshadow -Wpointer-arith -Wcast-qual -Wstrict-prototypes -Wmissing-prototypes -IFFHT
VPATH += FFHT

all: install

TARGET += test_quick test_neon
TARGET += test_float test_double test_float_header_only test_double_header_only

OBJ := fast_copy.o fht.o

install:
	git submodule update --init
	ln -s FFHT/fht.c fht.c
	python setup.py install --user

test_quick: test_quick.c $(OBJ)
	$(CC) $< $(OBJ) -o $@ $(CFLAGS) -lm

test_neon: test_neon.c $(OBJ)
	$(CC) $< $(OBJ) -o $@ $(CFLAGS) -lm

# Regular tests from FFHT (need object files)
test_double: test_double.c $(OBJ)
	$(CC) $< $(OBJ) -o $@ $(CFLAGS) -lm

test_float: test_float.c $(OBJ)
	$(CC) $< $(OBJ) -o $@ $(CFLAGS) -lm

# Header-only tests from FFHT (self-contained, no linking needed)
test_double_header_only: test_double_header_only.c
	$(CC) $< -o $@ $(CFLAGS)

test_float_header_only: test_float_header_only.c
	$(CC) $< -o $@ $(CFLAGS)

# Build all test executables
test: $(TARGET)
	@echo "All tests compiled successfully!"
	@echo "Run individual tests:"
	@echo "  ./test_quick        - Quick comprehensive test (ffht.rs)"
	@echo "  ./test_neon         - NEON-specific test (ffht.rs)"
	@echo "  ./test_float        - Float FHT test (FFHT)"
	@echo "  ./test_double       - Double FHT test (FFHT)"
	@echo "  ./test_float_header_only  - Float header-only test (FFHT)"
	@echo "  ./test_double_header_only - Double header-only test (FFHT)"

clean:
	rm -f $(OBJ) $(TARGET)
	rm -f fht.c
	rm -rf build/ FFHT.egg-info/ dist/

.PHONY: all test clean install
