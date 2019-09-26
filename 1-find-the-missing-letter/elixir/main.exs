IO.puts "Hello world from Elixir"

fn main() {
  let letter = missing([]rune{'a', 'b', 'c', 'd', 'f', 'g', 'h'});
  let letter2 = missing([]rune{'O', 'Q', 'R', 'S'})
  println!(letter, letter2);
}

fn missing(input []rune) string {
  start := int(input[0])
  for i, j := 0, start; i < 32; i, j = i+1, j+1 {
    if int(input[i]) != j {
      return string(j)
    }
  }
  return ""
}
