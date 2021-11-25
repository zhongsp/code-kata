CC=gcc
CFLAGS=-I. -Wall
DEPS=stack.h

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

make: main.o stack.o
	$(CC) main.o stack.o $(CFLAGS)
