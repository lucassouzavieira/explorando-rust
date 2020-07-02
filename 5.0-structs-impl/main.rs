// Estruturas de Rust
// Structs e enums

use std::string::String;

// Traits
trait Speaker {
    fn get_speaker(&self) -> String;
}

#[derive(Debug)]
struct Techlive {
    edition: i32,
    speaker: String
}

impl Techlive {
    pub fn get_edition(&self) -> i32 {
        return self.edition;
    }

    pub fn print(&self) {
        println!("Edição: {} | Palestrante: {}", self.edition, self.speaker);
    }
}

impl Speaker for Techlive {
    fn get_speaker(&self) -> String {
        return self.speaker.to_string();
    }
}

pub fn main() {
    let last = Techlive{edition: 8, speaker: String::from("Alisson Gomes")};
    last.print();
    println!("Edicao {}", last.get_edition());
    println!("Palestrante {}", last.get_speaker());
}
  