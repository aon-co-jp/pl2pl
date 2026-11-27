#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>


int gcd(int a, int b) { return b == 0 ? a : gcd(b, a % b); }

double mean(double *v, int n) {
  double s = 0;
  for (int i = 0; i < n; i++) s += v[i];
  return s / n;
}

int count_vowels(const char *s) {
  int c = 0;
  for (int i = 0; s[i] != '\0'; i++) {
    char ch = s[i];
    if (ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u') c++;
  }
  return c;
}

void swap(int *a, int *b) {
  int t = *a;
  *a = *b;
  *b = t;
}

int main(void) {
  printf("gcd=%d\n", gcd(48, 18));
  double v[4] = {1.5, 2.5, 3.0, 5.0};
  printf("mean=%.3f sqrt=%.3f pow=%.1f\n", mean(v, 4), sqrt(16.0), pow(2.0, 10.0));
  printf("vowels=%d\n", count_vowels("education"));
  int x = 3, y = 8;
  swap(&x, &y);
  printf("x=%d y=%d\n", x, y);
  char buf[32];
  sprintf(buf, "%d-%s", 42, "ok");
  printf("%s len=%d\n", buf, (int)strlen(buf));
  int n = 5;
  int *arr = (int *)malloc(n * sizeof(int));
  for (int i = 0; i < n; i++) arr[i] = (i * 7) % 5;
  int mx = arr[0];
  for (int i = 1; i < n; i++) if (arr[i] > mx) mx = arr[i];
  printf("max=%d abs=%d\n", mx, abs(-9));
  free(arr);
  double d = 7 / 2;
  double e = 7 / 2.0;
  printf("%.1f %.1f %d\n", d, e, 7 % 3);
  return 0;
}
