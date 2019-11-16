use std::thread;
use std::sync::{Arc, Mutex};

struct Numero{
    value: Mutex<i32>
}

impl Numero{

    fn new(x: i32) -> Numero{
        Numero{value: Mutex::new(x)}
    }

    fn add(& self, x: i32){
        let mut safe_value = self.value.lock().unwrap();
        *safe_value += x;
    }

    fn lessthan(& self, x:i32) -> bool{
        let safe_value = self.value.lock().unwrap();
        return *safe_value < x;
    }

    fn show(& self){
        let safe_value = self.value.lock().unwrap();
        println!("{}", *safe_value);
    }
}

fn add_to_100(x: & Numero, n: i32){
    while x.lessthan(100){
        x.add(1);
        //println!("Thread {}", n);
        //x.show();
    }
}

fn main() {
    let x = Arc::new(Numero::new(0));
    let y = x.clone();
    let z = x.clone();

    let h1 = thread::spawn(move || {
        add_to_100(&x, 1);
    });

    let h2 = thread::spawn(move || {
        add_to_100(&y, 2);
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("Final");
    z.show();

}
