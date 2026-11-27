#include <stdio.h>
void setpp(int **pp, int *t) { *pp = t; }
int main(void) {
  int a = 1, b = 9;
  int *q = &a;
  setpp(&q, &b);
  *q = *q + 1;
  printf("%d %d\n", a, b);
  return 0;
}
