CC = gcc
CFLAGS = -O3 -march=native -std=c99 -pedantic -Wall -Wextra -Wshadow -Wpointer-arith -Wcast-qual -Wstrict-prototypes -Wmissing-prototypes

all: install

TARGET += test_quick test_neon
TARGET += test_float test_double test_float_header_only test_double_header_only

OBJ := fast_copy.o fht.o

install:
	git submodule update --init
	ln -sf FFHT/fht.c fht.c
	python setup.py install --user

# %: %.c $(OBJ)
# 	$(CC) $< $(OBJ) -o $@ $(CFLAGS) -lm

test_quick: test_quick.c $(OBJ)
	$(CC) $< $(OBJ) -o $@ $(CFLAGS) -lm

test_neon: test_neon.c $(OBJ)
	$(CC) $< $(OBJ) -o $@ $(CFLAGS) -lm

test_double: FFHT/test_double.c $(OBJ)
	ln -sf $< $@.c
	$(CC) $@.c $(OBJ) -o $@ $(CFLAGS) -lm
	rm $@.c

test_float: FFHT/test_float.c $(OBJ)
	ln -sf $< $@.c
	$(CC) $@.c $(OBJ) -o $@ $(CFLAGS) -lm
	rm $@.c

# Header-only tests from FFHT (self-contained, no linking needed)
test_double_header_only: FFHT/test_double_header_only.c
	ln -sf $< $@.c
	$(CC) $@.c -o $@ $(CFLAGS)
	rm $@.c

test_float_header_only: FFHT/test_float_header_only.c
	ln -sf $< $@.c
	$(CC) $@.c -o $@ $(CFLAGS)
	rm $@.c

test-init:
	ln -sf FFHT/fht.c fht.c
	ln -sf FFHT/fht.h fht.h
	ln -sf FFHT/fht_header_only.h fht_header_only.h
	ln -sf FFHT/fast_copy.h fast_copy.h
	ln -sf FFHT/fht_avx.c fht_avx.c
	ln -sf FFHT/fht_sse.c fht_sse.c
	ln -sf FFHT/test_float.c test_float.c
	ln -sf FFHT/test_float_header_only.c test_float_header_only.c
	ln -sf FFHT/test_double.c test_double.c
	ln -sf FFHT/test_double_header_only.c test_double_header_only.c
	$(CC) -c fast_copy.c -o fast_copy.o $(CFLAGS) -lm
	$(CC) -c fht.c       -o fht.o       $(CFLAGS) -lm

# Build all test executables
test: test-init $(TARGET)
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
	rm -f fht.c fht.h 
	rm -rf build/ FFHT.egg-info/ dist/

.PHONY: all test clean install
