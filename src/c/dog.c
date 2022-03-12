//# 🐶 IS CUTE THAN 😼

#include <stdio.h>
#include <stdlib.h>

#define FACT_SIZE 22
#define FILE_NAME "./dog.c"

int main(int argc, char *argv[]) {
  FILE *file;
  long fsize;
  char *buffer, *ptr, *fact;
  char c;
  unsigned int unit;

  if ((file = fopen(FILE_NAME, "r")) == NULL)
    exit(1);

  fseek(file, 0, SEEK_END);
  fsize = ftell(file);
  if ((buffer = malloc(fsize)) == NULL)
    exit(1);
  if ((fact = calloc(1, FACT_SIZE + 1)) == NULL)
    exit(1);
  fseek(file, 0, SEEK_SET);
  ptr = buffer;

  while ((c = fgetc(file)) != EOF) {
    *ptr++ = c;
  }

  unit = fsize / FACT_SIZE;
  for (int ix = 0; ix != FACT_SIZE; ++ix) {
    char x = 0;
    for (int jx = 0; jx != unit; ++jx) {
      x += buffer[ix * unit + jx];
    }
    fact[ix] = x;
    switch (ix) {
      case  0: x = 0xEE; break;
      case  1: x = 0x5F; break;
      case  2: x = 0xC1; break;
      case  3: x = 0x4F; break;
      case  4: x = 0x35; break;
      case  5: x = 0xCF; break;
      case  6: x = 0x00; break;
      case  7: x = 0x97; break;
      case  8: x = 0xBC; break;
      case  9: x = 0x1A; break;
      case 10: x = 0xE4; break;
      case 11: x = 0x73; break;
      case 12: x = 0xDA; break;
      case 13: x = 0x05; break;
      case 14: x = 0x2A; break;
      case 15: x = 0x49; break;
      case 16: x = 0x73; break;
      case 17: x = 0x90; break;
      case 18: x = 0x1D; break;
      case 19: x = 0x3E; break;
      case 20: x = 0xA5; break;   
      case 21: x = 0x22; break;
      default: exit(1);
    }
    fact[ix] += x;
  }

  puts(fact);

  return 0;
}
