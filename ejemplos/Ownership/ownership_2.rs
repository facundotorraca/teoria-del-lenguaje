use std::io;

fn main() -> io::Result<()> {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    //println!("{}, {}", r1, r2);

    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

   // println!("{}", r3);

    Ok(())
}
