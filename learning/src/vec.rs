fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let mut v1 = vec![1, 2, 3, 4, 5];
    let first = v1.get(0);
    let f = &v1[2];
    match first {
        Some(first) => println!("The first element is: {}", first),
        None => println!("The vector is empty."),
    }
    println!("The first element is: {}", f);
    v1.push(100);

    for i in &mut v1 {
        *i += 50;
        println!("{}", i);
    }
}
