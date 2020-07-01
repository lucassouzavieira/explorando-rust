// Exemplo Hello World com Rust

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let name = &args[1]; // &args[0] contem sempre o nome do arquivo sendo executado    

    println!("Hello {}!!", name);
}
