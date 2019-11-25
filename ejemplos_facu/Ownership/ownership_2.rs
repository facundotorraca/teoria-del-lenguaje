use std::io;

/*
fn get_random_string() -> &String {
    let string = String::from("Hola");
    return &string;
}*/


fn main() -> io::Result<()> {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    //let string = get_random_string();

    Ok(())
}
