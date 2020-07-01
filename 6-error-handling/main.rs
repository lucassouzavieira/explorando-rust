// Error handling
use std::string::String;

fn is_even(n: i32) -> Result<String, String> {
    if n % 2 == 0 {
        return Ok(String::from("n par"));
    }

    return Err(String::from("n impar (Aka descricao do erro)"));
}

// unwrap / expect
// panic!
pub fn main() {
    panic!("Erro nao recuperavel!");
    let res = is_even(3).expect("Erro!");
    println!("{}", res);
}