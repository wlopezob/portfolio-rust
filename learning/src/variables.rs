fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // Esto causará un error de compilación porque `x` es inmutable
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 
    println!("Constante: {}", THREE_HOURS_IN_SECONDS);

    let y = 10;

    let y = y + 5;
   
    {
        let y = y * 2;
        println!("El valor de x en el scope interno es: {}", y);
    }
    println!("El valor de x en el scope externo es: {}", y);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let t:f64 = -5 as f64/3 as f64;
    println!("truncated: {}, t: {}", truncated, t);
    println!("quotient: {}", quotient);
    println!("untruncated: {}", t);

    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {:?}", tup);

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Arrays
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
}