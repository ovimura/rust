fn main() {
    let a = 10;
    let b = 3.0;
    let c = a as f64 / (b);
    print!("c is {0:08.3}\na is {1}\nonce again, c is {0}", c, a);
}
