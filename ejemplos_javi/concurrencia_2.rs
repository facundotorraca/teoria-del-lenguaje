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

fn main() {
    let x = Arc::new(Numero::new(0));
    let y = x.clone();
    let z = x.clone();

    let h1 = thread::spawn(move || {
        while x.lessthan(100){
            x.add(1);
            println!("Thread 1");
            x.show();
        }
    });

    let h2 = thread::spawn(move || {
        while y.lessthan(100){
            y.add(1);
            println!("Thread 2");
            y.show();
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("Final");
    z.show();

}

/*fn main(){
    let x = Arc::new(Mutex::new(Numero{valor: 0}));
    let y = x.clone();
    let z = x.clone();

    let handler1 = thread::spawn(move || {
        let mut safe_x = x.lock().unwrap();
        while safe_x.lessthan(100){
            safe_x.add(1);
            safe_x.show();
        }
    });

    let handler2 = thread::spawn(move || {
        let mut safe_y = y.lock().unwrap();
        while safe_y.lessthan(100){
            safe_y.add(1);
            safe_y.show();
        }        
    });

    handler1.join().unwrap();
    handler2.join().unwrap();

    z.lock().unwrap().show();
}*/

