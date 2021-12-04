all: main.asm
	nasm -f elf -g -gdwarf main.asm
	ld -m elf_i386 -s -o main main.o

.PHONY: clean
clean:
	rm -f main.o main
