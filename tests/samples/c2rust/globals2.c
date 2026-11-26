#include <stdio.h>
int n = 100;
int get(void) { return n; }
int local(int k) {
  int n = k * 2;
  n += 1;
  return n;
}
int main(void) {
  printf("%d %d %d\n", get(), local(5), n);
  n = 7;
  printf("%d\n", get());
  return 0;
}
