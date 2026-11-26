#include <stdio.h>

int sq(int v) { return v * v; }
int twice(int v) { int r = v * 2; return r; }

int main(void) {
  int i = 0;
  do { i++; } while (i < 3);
  printf("i=%d\n", i);
  int s = 0;
  for (int k = 0; k < 4; k++) s += sq(k);
  printf("s=%d\n", s);
  if (s > 10) printf("big\n");
  else printf("small\n");
  if (i == 3) { printf("three\n"); } else { printf("other\n"); }
  int n = 5;
  do {
    n -= 2;
    printf("n=%d\n", n);
  } while (n > 0);
  while (n < 3) n++;
  printf("n=%d twice=%d\n", n, twice(n));
  return 0;
}
