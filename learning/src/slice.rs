fn main() {
    let first = first_word(&String::from("Hi, how are you?"));
    println!("First word ends at index: {}", first);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..];

    println!("The first word is: {}", world);
    println!("The second word is: {}", hello);

    println!("First word slice: {}", first_word_slice(&s));

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}