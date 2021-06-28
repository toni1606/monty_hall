use std::io;

pub fn read_input(data: &mut usize) {
	let mut raw_data = String::new();
	io::stdin().read_line(&mut raw_data).expect("Error while reading from stdin!");

	*data = raw_data.trim()
				.parse()
				.expect("Unable to parse inputed data!");
}

pub fn check_if_valid(data: &usize, range: &[usize]) -> bool {
	let mut out: bool = false;

	for element in range {
		if *data == *element {
			out = true;
		}
	}
	out
}