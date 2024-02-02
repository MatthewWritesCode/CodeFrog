CC = g++
CFLAGS = -Wall -Wextra -pedantic -std=c++11
LDFLAGS =

SRCS = $(wildcard *.c) $(wildcard *.cpp)

PROG = main

$(PROG): $(SRCS)
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

all: $(PROG)
	@

clean:
	rm -f $(PROG)

