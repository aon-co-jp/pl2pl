#include <stdio.h>

int counter = 0;
static int total = 10;
static int twice(int v) { return v * 2; }
double rate = 1.5;
int tbl[3] = {1, 2, 3};

void bump(void) { counter++; total += 2; }

int main(void) {
  bump(); bump();
  printf("%d %d %.1f %d\n", counter, total, rate, tbl[1]);
  printf("twice=%d\n", twice(total));
  return 0;
}
