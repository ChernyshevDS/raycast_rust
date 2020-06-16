use std::io;

//https://habr.com/ru/post/436790/

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(n) => {
            println!("{} bytes read", n);
        },
        Err(e) => {
            println!("error: {}", e);
        }
    }
    println!("Hello, world!");
}
