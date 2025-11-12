CC = gcc
CFLAGS = -O3 -march=native -std=c99 -pedantic -Wall -Wextra -Wshadow -Wpointer-arith -Wcast-qual -Wstrict-prototypes -Wmissing-prototypes

all: install

test: test_float test_double fast_copy.o fht.o

OBJ := fast_copy.o fht.o

install:
	git submodule update --init
	ln -s FFHT/fht.c fht.c
	python setup.py install --user

%.o: %.c
	$(CC) $< -o $@ -c $(CFLAGS)

test_%: test_%.c $(OBJ)
	$(CC) $< $(OBJ) -o $@ $(CFLAGS) -lm

test_neon: test_neon.c $(OBJ)
	$(CC) $< $(OBJ) -o $@ $(CFLAGS) -lm

test_double_header_only: test_double_header_only.c
	$(CC) $< -o $@ $(CFLAGS)

test_float_header_only: test_double_header_only.c
	$(CC) $< -o $@ $(CFLAGS)

clean:
	rm -f test_float test_double test_neon test_float_header_only test_double_header_only $(OBJ)
	rm -f fht.c
	rm -rf build/ FFHT.egg-info/

.PHONY: all test clean
