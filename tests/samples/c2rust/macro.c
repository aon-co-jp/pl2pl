#include <stdio.h>

#define SQ(x) ((x) * (x))
#define MAX2(a, b) ((a) > (b) ? (a) : (b))
#define ADD3(a, b, c) (a + b + c)

int main(void) {
  int n = 5;
  int s = SQ(n + 1);
  printf("sq=%d\n", s);
  printf("max=%d\n", MAX2(n, 9));
  printf("add=%d\n", ADD3(1, SQ(2), 3));
  double d = SQ(1.5);
  printf("d=%.2f\n", d);
  return 0;
}
