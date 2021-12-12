fn main() {
    ownership();
}

fn ownership() {
    let s = "A test string";
    let v = s;
    println!("v: {}", v);
    println!("s: {}", s);
}