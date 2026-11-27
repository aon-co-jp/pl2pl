#include <stdio.h>
#include <stdlib.h>
#include <string.h>
struct P { char name[16]; int age; };
int by_age(const void *a, const void *b) {
  const struct P *x = (const struct P *)a;
  const struct P *y = (const struct P *)b;
  return x->age - y->age;
}
int by_name(const void *a, const void *b) {
  return strcmp(((const struct P *)a)->name, ((const struct P *)b)->name);
}
int main(void) {
  struct P v[4];
  strcpy(v[0].name, "zed"); v[0].age = 30;
  strcpy(v[1].name, "amy"); v[1].age = 41;
  strcpy(v[2].name, "bob"); v[2].age = 22;
  strcpy(v[3].name, "cat"); v[3].age = 35;
  qsort(v, 4, sizeof(struct P), by_age);
  for (int i = 0; i < 4; i++) printf("%s %d\n", v[i].name, v[i].age);
  qsort(v, 4, sizeof(struct P), by_name);
  for (int i = 0; i < 4; i++) printf("%s %d\n", v[i].name, v[i].age);
  return 0;
}
