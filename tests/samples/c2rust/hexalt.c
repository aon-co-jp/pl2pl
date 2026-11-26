#include <stdio.h>

int main(void) {
  int a = 255, z = 0, b = 8;
  printf("%#x %#X %#o\n", a, a, b);
  printf("zero=%#x %#o\n", z, z);
  printf("[%#6x] [%-8d] [%#-6x]\n", 171, a, 171);
  printf("%#06x %#06o %#08X|%#04x\n", 171, 8, 171, 1);
  return 0;
}
