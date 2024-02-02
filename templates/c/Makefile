CC = gcc
CFLAGS = -Wall -Wextra -pedantic
LDFLAGS =

SRCS = $(wildcard *.c) $(wildcard *.cpp)

PROG = main

$(PROG): $(SRCS)
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

all: $(PROG)
	@

clean:
	rm -f $(PROG)
