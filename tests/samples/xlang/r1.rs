fn main() {
    let mut s: i64 = 0;
    let n = 5;
    let mut x = 0.5;
    for i in 1..=n {
        if i % 2 == 1 {
            s += i as i64 * 3_000_000_000;
        } else {
            x *= 2.0;
        }
    }
    let mut k = 0;
    while k < 3 {
        print!("k{} ", k);
        k += 1;
    }
    println!();
    let ok = s > 10;
    println!("s={} x={:.3} n={:4} ok={} half={}", s, x, n, ok, x / 4.0);
    println!("{{done}}");
}
