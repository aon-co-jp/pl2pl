#include <stdio.h>

int main(void) {
  for (int i = 0; i < 6; i++) {
    switch (i) {
      case 0:
        printf("zero\n");
        break;
      case 1:
      case 2:
        printf("one-two %d\n", i);
        break;
      case 3: printf("three\n"); break;
      case 4:
        for (int k = 0; k < 2; k++) {
          if (k == 1) break;
          printf("k=%d\n", k);
        }
        break;
      default:
        printf("other %d\n", i);
    }
  }
  char c = 'b';
  switch (c) {
    case 'a': printf("A\n"); break;
    case 'b': printf("B\n"); break;
  }
  int n = 9;
  switch (n) {
    case 1: printf("never\n"); break;
  }
  printf("done\n");
  return 0;
}
