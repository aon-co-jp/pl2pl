#include <stdio.h>

enum Color { RED, GREEN = 5, BLUE };
typedef enum { LOW = 1, MID, HIGH } Level;

int score(Level l) {
  switch (l) {
    case LOW: return 10;
    case MID: return 20;
    default: return 30;
  }
}

int main(void) {
  enum Color c = GREEN;
  Level lv = MID;
  printf("c=%d blue=%d red=%d\n", c, BLUE, RED);
  printf("lv=%d high=%d\n", lv, HIGH);
  printf("%d %d %d\n", score(LOW), score(lv), score(HIGH));
  if (c == GREEN) printf("green\n");
  return 0;
}
