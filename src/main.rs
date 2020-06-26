fn main() {
    let x = vec!["Hello", "World"];
    let y: Vec<_> = x.iter().rev().collect();
    println!("{:?}", y);
}
