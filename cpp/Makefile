CC=c++
CC_OPTS=-O2 -std=c++17 -Wall -Werror

all: build/main build/benchmark

run: build/main
	build/main ../input.txt

benchmark: build/benchmark
	build/benchmark

build/main: Makefile build/obj/read_file.o build/obj/impl.o
	$(CC) $(CC_OPTS) build/obj/read_file.o build/obj/impl.o main.cpp -o $@

build/obj/impl.o: Makefile impl.h impl.cpp
	@mkdir -p build/obj/
	$(CC) $(CC_OPTS) -c -o $@ impl.cpp

build/obj/read_file.o: Makefile read_file.h read_file.cpp
	@mkdir -p build/obj/
	$(CC) $(CC_OPTS) -c -o $@ read_file.cpp

build/benchmark: Makefile benchmark.cpp build/obj/read_file.o build/obj/impl.o
	$(CC) $(CC_OPTS) build/obj/read_file.o build/obj/impl.o benchmark.cpp -o $@