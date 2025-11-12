CC = gcc
CFLAGS = -O3 -march=native -std=c99 -pedantic -Wall -Wextra -Wshadow -Wpointer-arith -Wcast-qual -Wstrict-prototypes -Wmissing-prototypes
CFLAGS += -DFHT_HEADER_ONLY  # This macro is used in setup.py

all: install

TARGET += test_quick test_neon test_float test_double

create-link:
	ln -sf FFHT/fht_avx.c fht_avx.c
	ln -sf FFHT/fht_sse.c fht_sse.c

install: create-link
	git submodule update --init
	python setup.py install --user

test_quick: test_quick.c fht.c
	$(CC) $^ -o $@ $(CFLAGS) -lm

test_neon: test_neon.c fht.c
	$(CC) $^ -o $@ $(CFLAGS) -lm

test_double: FFHT/test_double.c fht.c
	$(CC) $^ -o $@ $(CFLAGS) -lm

test_float: FFHT/test_float.c fht.c
	$(CC) $^ -o $@ $(CFLAGS) -lm

# Build all test executables
test: create-link $(TARGET)
	@echo "All tests compiled successfully!"
	@echo "Run individual tests:"
	@echo "  ./test_quick        - Quick comprehensive test (ffht.rs)"
	@echo "  ./test_neon         - NEON-specific test (ffht.rs)"
	@echo "  ./test_float        - Float FHT test (FFHT)"
	@echo "  ./test_double       - Double FHT test (FFHT)"

clean:
	rm -f $(OBJ) $(TARGET) fht.o
	rm -f fht_avx.c fht_sse.c
	rm -rf build/ FFHT.egg-info/ dist/

.PHONY: all test clean install create-link
