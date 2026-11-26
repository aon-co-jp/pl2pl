#include <stdio.h>
int main(void) {
    int s = 0, n = 10;
    double x = 1.5;
    long big = 1;
    for (int i = 0; i < n; i++) {
        if (i % 2 == 0 && i != 4) {
            s += i;
        } else if (i > 7) {
            s -= 1;
        } else {
            x = x * 2.0;
        }
        big *= 3;
    }
    for (int j = 9; j >= 0; j -= 3) {
        printf("j=%d ", j);
    }
    printf("\n");
    while (n > 0) {
        n -= 3;
    }
    printf("s=%d x=%.2f n=%5d big=%ld\n", s, x, n, big);
    puts("done 100%");
    return 0;
}
