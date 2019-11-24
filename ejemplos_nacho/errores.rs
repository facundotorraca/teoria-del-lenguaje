fn div(x: i32, y:i32) -> Result<i32, &'static str>{
    if y == 0{
        return Err("zero division");
    }
    return Ok(x/y);
}

fn main(){
    match div(5,0){
        Ok(num) => println!("la division dio {}", num),
        Err(e) => println!("{}", e)
    }
}
