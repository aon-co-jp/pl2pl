#include <stdio.h>

int main(void) {
  int a = 12, b = 10;
  printf("%d %d %d\n", a & b, a | b, a ^ b);
  printf("%d %d\n", a << 2, a >> 1);
  printf("%d\n", ~a);
  int f = 0;
  f |= 1;
  f |= 1 << 3;
  f &= ~1;
  f ^= 2;
  f <<= 1;
  f >>= 1;
  printf("f=%d\n", f);
  if ((f & 8) != 0) printf("bit3\n");
  printf("%d %d\n", 0xFF & 0x0F, (a + 1) << (b - 8));
  return 0;
}
