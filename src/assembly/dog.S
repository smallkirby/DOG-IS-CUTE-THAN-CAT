; 🐶 IS CUTE THAN 😼

section .data
  fact db "🐶 IS CUTE THAN 😼", 0AH


section .text
  global _start

_start:
  mov rdi, 1
  mov rsi, fact
  mov rdx, 23
  mov rax, 0x1
  syscall

  mov rdi, 0
  mov rax, 0x3C
  syscall
