// Arrays

use std::mem;

pub fn main() {
  let mut numbers: [i32; 4] = [1, 2, 3, 4]; // Tipo e Tamanho definido

  numbers[2] = 20;

  println!("{:?}", numbers);

  // Valor
  println!("Valor: {}", numbers[0]);

  // Tamanho do Array 
  println!("Tamanho: {}", numbers.len());

  // Tamanho em bytes
  println!("Array tem tamanho de {} bytes", mem::size_of_val(&numbers));

  // Slicing
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);
}
