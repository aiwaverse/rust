fn main() {
    ownership();
}

fn ownership() {
    let s1 = String::from("A test string");
    let s2 = s1.clone();
    println!("s1: {:p}", &s1);
    println!("s2: {:p}", &s2);

    let v1 = vec![1, 2, 3];
    let mut v2 = v1.clone();
    v2[0] = 5;
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
}