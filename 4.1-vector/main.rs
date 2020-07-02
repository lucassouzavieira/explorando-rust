// Vectors

use std::mem;

pub fn main() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  // Adicionando elementos
  numbers.push(5);
  numbers.push(6);

  println!("{:?}", numbers);

  // Ultimo valor
  let last: i32 = numbers.pop().unwrap();
  println!("Ultimo valor (pop): {}", last);

  println!("{:?}", numbers);

  // Valor
  println!("Valor: {}", numbers[0]);

  // Tamanho do Vetor 
  println!("Tamanho: {}", numbers.len());

  // Tamanho em bytes
  println!("Vetor tem tamanho de {} bytes", mem::size_of_val(&numbers));

  // Slicing
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);

  // Maneiras de iterar um vetor
  // Imutavel
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Mutavel
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers Vec: {:?}", numbers);
}
