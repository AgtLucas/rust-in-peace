fn main() {

  // Int calc?
  let program = "+ + * - /";
  let mut accumulator = 0i;

  for token in program.chars() {
    match toke {
      '+' => accumulator += 1,
      '-' => accumulator -= 1,
      '*' => accumulator *= 2,
      '/' => accumulator /= 2,
      _ => { /* Ignore everything else */ }
    }
  }

  println!("The program \"{}\" calculates the value {}"), program, accumulator);

}