// Enums

// Se comportam quase como um tipo
// Tem comportamento diferente das enums em C
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

// Atencao no parametro
fn print_move(m: Movement) {
    match m {
        Movement::Up => println!("Acima"),
        Movement::Down => println!("Abaixo"),
        Movement::Left => println!("Esquerda"),
        Movement::Right => println!("Direita"),
    }
}

pub fn main() {
    print_move(Movement::Up);
    print_move(Movement::Down);
    print_move(Movement::Left);
    print_move(Movement::Right);
}