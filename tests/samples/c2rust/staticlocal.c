#include <stdio.h>
int next(void) {
  static int n = 0;
  n++;
  return n;
}
int main(void) {
  int a = next();
  int b = next();
  printf("%d %d %d\n", a, b, next());
  return 0;
}
