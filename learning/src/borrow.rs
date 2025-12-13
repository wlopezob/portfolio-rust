fn main(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);

    let mut s3 = String::from("hello");
    {
        let r1 = &mut s3;
        r1.push_str(", world");
        println!("{}", r1);
    }
    println!("{}", s3);

    // el borrow not allow to have mutable and immutable references at the same time
    let mut s4 = String::from("hello 4");
    let s41 =&s4;
    let s42= &s4;
    println!("{},{}",s41,s42);

    let s45 = &mut s4;
    println!("{}",s45);
    s45.push_str("hola");
    // let s5 = &
}

fn calculate_length(s:&String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}