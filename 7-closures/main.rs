// Closures

// Funcao retornando closure sem retorno
fn get_printer() -> impl Fn(i32) {
    return |num: i32| {
        println!("From printer: {}", num);
    }
}

// Square
fn get_square() -> impl Fn(i32) -> i32 {
    return |num: i32| {
        num * num
    }
}

pub fn main() {
    let double_number = |num: i32| -> i32 {
        num * 2
    };

    println!("Closure inline: {}", double_number(10));

    // Printer
    let printer = get_printer();
    printer(10);

    // Square
    let square = get_square();
    printer(square(5));
}