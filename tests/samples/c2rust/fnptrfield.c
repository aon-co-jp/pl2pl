#include <stdio.h>
int add(int a, int b) { return a + b; }
int mul(int a, int b) { return a * b; }
struct Op { char sym; int (*f)(int, int); };
int main(void) {
  struct Op ops[2];
  ops[0].sym = '+'; ops[0].f = add;
  ops[1].sym = '*'; ops[1].f = mul;
  for (int i = 0; i < 2; i++) printf("%c %d\n", ops[i].sym, ops[i].f(6, 7));
  int (*tab[2])(int, int) = {add, mul};
  printf("%d %d\n", tab[0](2, 3), tab[1](2, 3));
  return 0;
}
