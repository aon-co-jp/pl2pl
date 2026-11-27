#include <stdio.h>
struct P { int x; int y; };
void setp(struct P **pp, struct P *t) { *pp = t; }
int main(void) {
  struct P a = {1, 2};
  struct P b = {7, 8};
  struct P *q = &a;
  setp(&q, &b);
  q->x = q->x + 1;
  printf("%d %d %d\n", a.x, b.x, q->y);
  return 0;
}
