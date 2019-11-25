use std::io;

/*str es el tipo nativo de string  */
/* no es el objeto y son inmutables*/
fn find_tdl_in_string(string : &String) -> &str {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'L' {
            return &string[0..i+1];
        }
    }

    &string[..]
}

fn main() -> io::Result<()> {

    let mut string = String::from("TDLLLLLLLLLL");

    let tdl = find_tdl_in_string(&string);

    println!("{}", tdl);

    string.push_str("HOLA");
    println!("{}", string);

    return Ok(())
}