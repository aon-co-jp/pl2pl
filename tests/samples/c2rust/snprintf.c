#include <stdio.h>
int main(void) {
  char b[32];
  snprintf(b, 32, "%d-%s", 7, "x");
  printf("%s\n", b);
  return 0;
}

