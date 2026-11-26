#include <stdio.h>
#include <string.h>

int fib(int n) {
  if (n < 2) return n;
  return fib(n - 1) + fib(n - 2);
}

int is_prime(int n) {
  if (n < 2) return 0;
  for (int i = 2; i * i <= n; i++) {
    if (n % i == 0) return 0;
  }
  return 1;
}

void bubble(int a[], int n) {
  for (int i = 0; i < n - 1; i++)
    for (int j = 0; j < n - 1 - i; j++)
      if (a[j] > a[j + 1]) {
        int t = a[j];
        a[j] = a[j + 1];
        a[j + 1] = t;
      }
}

void reverse(char *s) {
  int n = strlen(s);
  for (int i = 0; i < n / 2; i++) {
    char t = s[i];
    s[i] = s[n - 1 - i];
    s[n - 1 - i] = t;
  }
}

int main(void) {
  for (int i = 0; i < 8; i++) printf("%d ", fib(i));
  printf("\n");
  int cnt = 0;
  for (int i = 0; i < 50; i++) if (is_prime(i)) cnt++;
  printf("primes=%d\n", cnt);
  int a[6] = {5, 2, 9, 1, 7, 3};
  bubble(a, 6);
  for (int i = 0; i < 6; i++) printf("%d%c", a[i], i < 5 ? ',' : '\n');
  char s[16] = "hello";
  reverse(s);
  printf("%s %d\n", s, (int)strlen(s));
  long big = 1L << 40;
  printf("%ld\n", big);
  unsigned char uc = 250;
  uc += 10;
  printf("%d\n", uc);
  return 0;
}
