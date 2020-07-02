// Error handling
use std::string::String;

fn is_even(n: i32) -> Result<String, String> {
    if n % 2 == 0 {
        return Ok(String::from("n par"));
    }

    return Err(String::from("n impar (Aka descricao do erro)"));
}

// unwrap / expect
pub fn main() {
    // panic!("Erro nao recuperavel!");
    let res = is_even(6);

    match res {
        Ok(res) => {
            println!("Ok");
        },
        Err(res) => {
            println!("Tratamento");
        }
    }

    println!("Fim do programa");
}