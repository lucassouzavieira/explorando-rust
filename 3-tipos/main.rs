//
// Tipos primitivos
// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
// Floats: f32, f64
// Boolean
// Characters
// Tuples
// Arrays

/*
 Rust é estaticamente tipada - o compilador deve saber o tipo de todas as variaveis em tempo de compilacao.
 Mas o compilador consegue inferir alguns tipos.
*/

pub fn main() {
    let face = '\u{1F600}'; // Exemplo emoji.
  
    println!("{:?}", face);
}
  