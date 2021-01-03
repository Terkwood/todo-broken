fn main() {
    let der = "DERP dogs are cute";
    let mut n = 0;
    while n < 10000000 {
        println!("{} ... {}", der, n);
        n = n + 1
    }
}
