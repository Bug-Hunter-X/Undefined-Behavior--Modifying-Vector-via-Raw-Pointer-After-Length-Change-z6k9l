fn main() {
    let mut v = vec![1, 2, 3];
    // Safe way to modify the first element
    v[0] = 4;
    println!("{:?}", v);
}
