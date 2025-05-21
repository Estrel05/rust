fn main() {
    let mut s = String::from("hello"); // heap data
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1; // move
    let s3 = s2.clone(); // copy

    // println!("{}", s1); -> invalid
    println!("{}", s2); // -> valid
    println!("{}", s3);
}
