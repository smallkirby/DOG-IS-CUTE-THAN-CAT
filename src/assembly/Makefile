fact: dog
	./$<

.INTERMEDIATE: dog
.PHONY: dog
dog: dog.o
	ld $@.o -o $@

.INTERMEDIATE: dog.o
.PHONY: dog.o
dog.o: dog.S
	nasm -f elf64 $< -o $@

.PHONY: clean
clean:
	rm dog.o dog
