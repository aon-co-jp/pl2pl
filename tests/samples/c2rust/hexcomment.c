#include <stdio.h>

/* block comment before main
   spanning two lines */
int main(void) {
  int a = 255; /* trailing block comment */
  int b = 8;
  printf("hex=%x HEX=%X oct=%o\n", a, a, b);
  printf("pad=%02x w=%4x\n", 10, 171);
  printf("text with /* not a comment */ stays %d\n", a);
  return 0;
}
