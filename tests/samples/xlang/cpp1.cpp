#include <iostream>
using namespace std;
int main() {
    int n = 3;
    long total = 0;
    double x = 2.5;
    bool ok = true;
    const char *s = "hi";
    int *p = nullptr;
    for (int i = 1; i <= n; i++) {
        total += i * 1000;
    }
    cout << "n=" << n << " x=" << x * 2 << " ok=" << ok << " " << s << endl;
    cout << "total=" << total << " half=" << x / 3 << '\n';
    if (p == nullptr) {
        std::printf("null %d\n", n);
    }
    return 0;
}
