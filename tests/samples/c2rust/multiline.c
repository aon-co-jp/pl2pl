#include <stdio.h>
int add(int a,
        int b) {
  return a + b;
}
int main(void) {
  int x = add(1,
              2);
  printf("%d %d\n",
         x,
         add(x, 4));
  return 0;
}
