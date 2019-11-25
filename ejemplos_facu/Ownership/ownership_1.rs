use std::io;

/*--------------------------------------*/
/*Rust pasa todas las cosas por defecto */
/*como movimiento, no dejandote acceder */
/*despues de moverlas en tiempo de      */
/*compilacion                           */
/*--------------------------------------*/

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


fn funcion_que_hace_algo_con_string(string: String) {
    string.push('!');
    println!("{}", &string);
}

fn funcion_que_hace_algo_con_numero(numero: Numero) {
    numero.imprimir();
    numero.imprimir();
}

fn main() -> io::Result<()> {
    let mut string = String::from("Hola, soy un string");
    funcion_que_hace_algo_con_string(string);
    //println!("{}", string);


    let numero = Numero::new(1);
    funcion_que_hace_algo_con_numero(numero);
    //numero.imprimir();

    return Ok(());
}
