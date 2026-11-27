#include <stdio.h>
int twice(int x) { return x * 2; }
int sq(int x) { return x * x; }
int apply(int (*f)(int), int v) { return f(v); }
int main(void) {
  int (*g)(int) = sq;
  printf("%d %d %d\n", apply(twice, 4), apply(sq, 5), g(3));
  return 0;
}
