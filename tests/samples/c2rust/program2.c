#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Node {
  int val;
  struct Node *next;
};

struct Node *push(struct Node *head, int v) {
  struct Node *n = (struct Node *)malloc(sizeof(struct Node));
  n->val = v;
  n->next = head;
  return n;
}

int sum(struct Node *h) {
  int s = 0;
  while (h != NULL) {
    s += h->val;
    h = h->next;
  }
  return s;
}

struct Item {
  char name[16];
  int qty;
  double price;
};

double total(struct Item items[], int n) {
  double t = 0.0;
  for (int i = 0; i < n; i++) t += items[i].qty * items[i].price;
  return t;
}

int main(void) {
  struct Node *head = NULL;
  for (int i = 1; i <= 4; i++) head = push(head, i * 10);
  printf("sum=%d\n", sum(head));
  struct Item items[2];
  strcpy(items[0].name, "pen");
  items[0].qty = 3;
  items[0].price = 1.5;
  strcpy(items[1].name, "book");
  items[1].qty = 2;
  items[1].price = 12.25;
  printf("total=%.2f\n", total(items, 2));
  printf("%s %s\n", items[0].name, items[1].name);
  int grid[3][3];
  for (int r = 0; r < 3; r++)
    for (int c = 0; c < 3; c++)
      grid[r][c] = r * 3 + c;
  int tr = 0;
  for (int k = 0; k < 3; k++) tr += grid[k][k];
  printf("trace=%d\n", tr);
  if (strcmp(items[0].name, "pen") == 0) printf("same\n");
  return 0;
}
