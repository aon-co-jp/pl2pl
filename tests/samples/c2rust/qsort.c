#include <stdio.h>
#include <stdlib.h>
int cmp(const void *a, const void *b) {
  return *(const int *)a - *(const int *)b;
}
int main(void) {
  int v[5] = {5, 2, 9, 1, 7};
  qsort(v, 5, sizeof(int), cmp);
  for (int i = 0; i < 5; i++) printf("%d ", v[i]);
  printf("\n");
  return 0;
}
