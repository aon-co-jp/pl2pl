#include <stdio.h>
struct Node { int v; struct Node *next; };
int main(void) {
  struct Node a; struct Node *head = &a;
  a.v = 3; a.next = NULL;
  int s = 0;
  for (struct Node *p = head; p != NULL; p = p->next) {
    s += p->v;
  }
  printf("%d\n", s);
  return 0;
}
