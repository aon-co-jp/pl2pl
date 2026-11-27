#include <stdio.h>
#include <stdlib.h>
struct Node { int v; struct Node *next; };
int main(void) {
  struct Node *head = NULL;
  for (int i = 1; i <= 3; i++) {
    struct Node *n = malloc(sizeof(struct Node));
    n->v = i * 10;
    n->next = head;
    head = n;
  }
  int sum = 0;
  for (struct Node *p = head; p != NULL; p = p->next) sum += p->v;
  printf("sum=%d first=%d\n", sum, head->v);
  while (head) { struct Node *t = head->next; free(head); head = t; }
  return 0;
}
