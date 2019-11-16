use std::{io, thread};
use std::sync::{Arc, Mutex};

struct Numero {
    valor : Box<i32>
}

impl Numero {

    pub fn new(numero : i32) -> Numero {
        Numero{ valor: Box::new(numero) }
    }

    pub fn imprimir(&self) {
        println!("El numero es: {}", &self.valor);
    }

}

/*
fn move_from_to(mut from : Vec<Numero>, mut to : Vec<Numero>) {
    while !from.is_empty() {
        if let Some(numero) = from.pop() {
            to.push(numero);
        }
    }
}
*/

/*
fn move_from_to(from : Arc<Vec<Numero>>, to : Arc<Vec<Numero>>) {
    while !from.is_empty() {
        if let Some(numero) = from.pop() {
            to.push(numero);
        }
    }
}
*/

fn move_from_to(from : &Arc<Mutex<Vec<Numero>>>, to : &Arc<Mutex<Vec<Numero>>>, id: i32) {
    println!("By {}", id);
    if let Some(numero)= from.lock().unwrap().pop() {
        to.lock().unwrap().push(numero);
    }
}


fn main() -> io::Result<()> {
    let to: Vec<Numero> = Vec::new();

    let from = vec!{Numero::new(0),
                                 Numero::new(1),
                                 Numero::new(2),
                                 Numero::new(3),
                                 Numero::new(4),
                                 Numero::new(5)};


    //let sharable_to = Arc::new(to);
    //let sharable_from = Arc::new(from);

    let protected_sharable_to = Arc::new(Mutex::new(to));
    let protected_sharable_from = Arc::new(Mutex::new(from));

    let copy_to_a = protected_sharable_to.clone();
    let copy_to_b = protected_sharable_to.clone();

    let copy_from_a = protected_sharable_from.clone();
    let copy_from_b = protected_sharable_from.clone();


    let handler_a = thread::spawn(move || { while !copy_from_a.lock().unwrap().is_empty() {
                                                                 move_from_to(&copy_from_a, &copy_to_a, 1);
                                                             }
    });

    let handler_b = thread::spawn(move || { while !copy_from_b.lock().unwrap().is_empty() {
                                                                move_from_to(&copy_from_b, &copy_to_b, 2);
                                                            }
    });

    handler_a.join().expect("Join Error");
    handler_b.join().expect("Join Error");

    for numero in protected_sharable_to.lock().unwrap().iter() {
        numero.imprimir();
    }

    return Ok(());
}