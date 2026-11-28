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
    let a = 0.1 + 0.2;
    let b = -0.5;
    let c = 3.0;
    let d = 1.0e20;
    let e = 1.0 / 3.0;
    println!("{} {} {} {} {}", a, b, c, d, e);
    for i in (0..5).rev() {
        print!("{} ", i);
    }
    for i in (1..=10).step_by(3) {
        print!("{} ", i);
    }
    for i in (2..9).step_by(2) {
        print!("{} ", i);
    }
    for i in (0..=3).rev() {
        print!("{} ", i);
    }
    println!();
    println!("{{done}}");
}
