use std::io;

enum State {
	Closed,
	Open
}

struct Door {
	state: State,
	has_prize: bool
}

impl Door {
	fn new() -> Door {
		Door {
			state: State::Closed,
			has_prize: false 
		}
	}
	fn is_open(&self) -> bool {
		match self.state {
			State::Open		=> true,
			State::Closed	=> false
		}
	}
	fn open(&mut self) {
		self.state = State::Open;
	}
}

fn main() {
    println!("Hello, world!");
	let mut doors: [Door; 3] = [Door::new(), Door::new(), Door::new()];
	let mut choice: u8 = 0;

	loop {
		println!("Which door do you choose? (1, 2, 3)");
		read_input(&mut choice);
		
		if check_if_valid(&choice) {
			break;
		}
		println!("Invalid number entered!");
	}
}

fn read_input(data: &mut u8) {
	let mut raw_data = String::new();
	io::stdin().read_line(&mut raw_data).expect("Error while reading from stdin!");

	*data = raw_data.trim()
				.parse()
				.expect("Unable to parse inputed data!");
}

fn check_if_valid(data: &u8) -> bool {
	if *data >=1u8 && *data <= 3u8 {
		true
	} else {
		false
	}
}
