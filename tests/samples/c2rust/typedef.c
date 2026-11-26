#include <stdio.h>

typedef struct { int x; int y; } Pt;
typedef struct Rect {
  Pt a;
  Pt b;
  double scale;
} Rect;

int area(Rect r) {
  return (r.b.x - r.a.x) * (r.b.y - r.a.y);
}

void shift(Pt *p, int dx) {
  p->x += dx;
}

int main(void) {
  Pt p = {3, 4};
  Rect r;
  r.a.x = 0;
  r.a.y = 0;
  r.b.x = 5;
  r.b.y = 2;
  r.scale = 1.5;
  shift(&p, 10);
  printf("p=%d,%d\n", p.x, p.y);
  printf("area=%d scale=%.1f\n", area(r), r.scale);
  return 0;
}
