fn main() {
	let input1: [char; 7] = ['a', 'b', 'c', 'd', 'f', 'g', 'h'];
	let input2: [char; 4] = ['O', 'Q', 'R', 'S'];
    let letter1 = missing(& input1);
	let letter2 = missing(& input2);
	println!("{} {}", letter1, letter2);
}

fn missing(input: & [char]) -> char {
	let start = input[0] as u8;

	let mut i = 0;
	let mut j = start;
	while i < 32 {
		if input[i] as u8 != j {
			return j as char;
		}

		i += 1; j += 1;
	}
	return ' ';
}
