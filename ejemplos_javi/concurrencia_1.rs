use std::thread;
use std::sync::{Arc, Mutex};

struct Numero{
    value: i32
}

impl Numero{

    fn new(x: i32) -> Numero{
        Numero{value: x}
    }

    fn add(& mut self, x: i32){
        self.value += x;
    }

    fn lessthan(& self, x:i32) -> bool{
        return self.value < x;
    }

    fn show(& self){
        println!("{}", self.value);
    }
}

fn main(){
    let x = Arc::new(Mutex::new(Numero::new(0)));
    let y = x.clone();
    let z = x.clone();

    let h1 = thread::spawn(move || {
        let mut x_safe = x.lock().unwrap();
        while x_safe.lessthan(100){
            x_safe.add(1);
            println!("Thread 1");
            x_safe.show();
        }
    });

    let h2 = thread::spawn(move || {
        let mut y_safe = y.lock().unwrap();
        while y_safe.lessthan(100){
            y_safe.add(1);
            println!("Thread 2");
            y_safe.show();
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();

    z.lock().unwrap().show();

}

/*fn main(){
    let x = Arc::new(Mutex::new(Numero::new(0)));
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

