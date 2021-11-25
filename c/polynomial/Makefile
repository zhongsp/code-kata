CC=gcc
CFLAGS=-I. -Wall
DEPS=polynomial.h

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

make: main.o polynomial.o
	$(CC) main.o polynomial.o $(CFLAGS)
