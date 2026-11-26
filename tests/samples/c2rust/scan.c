#include <stdio.h>

int main(void) {
  int a, b;
  double d;
  long n;
  scanf("%d %d", &a, &b);
  scanf("%lf", &d);
  scanf("%ld", &n);
  printf("sum=%d prod=%d\n", a + b, a * b);
  printf("d2=%.2f n=%ld\n", d * 2, n);
  return 0;
}
