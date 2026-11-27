#include <stdio.h>
#include <string.h>
struct P { char name[16]; int age; };
int main(void) {
  struct P ps[2];
  strcpy(ps[0].name, "ann");
  ps[0].age = 30;
  strcpy(ps[1].name, "bob");
  ps[1].age = 25;
  int total = 0;
  for (int i = 0; i < 2; i++) {
    total += ps[i].age;
    printf("%s:%d len=%d\n", ps[i].name, ps[i].age, (int)strlen(ps[i].name));
  }
  printf("total=%d cmp=%d\n", total, strcmp(ps[0].name, ps[1].name) < 0);
  return 0;
}
