#include <stdio.h>
#include <string.h>
void pick(char **out, char *a, char *b, int w) { if (w) *out = a; else *out = b; }
int main(void) {
  char x[8] = "left";
  char y[8] = "right";
  char *s = x;
  pick(&s, x, y, 0);
  printf("%s %d\n", s, (int)strlen(s));
  pick(&s, x, y, 1);
  printf("%s\n", s);
  return 0;
}
