use std::{fs::File, io::{self, ErrorKind, Read}, net::IpAddr};

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let f = File::open("hello.txt");

    let greeting_file = match f  {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            },
        },
    };

    // let f2 = File::open("hello01.txt")
    //    .expect("Hello.txt should be included in this project");

    let f3 = File::open("hello03.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello03.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // let f4 = File::open("hello04.txt").unwrap();
    let username = read_username_from_file();
    let txt = username.unwrap_or("No username found".to_string());
    println!("Content file: {}", txt);

    let home: IpAddr = "127.0.0.1".parse()
        .expect("Failed to parse IP address");  
    println!("Home IP address: {}", home);
}

fn read_user_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),       
    }

    

}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello05.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello05.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}