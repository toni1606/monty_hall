use std::io;
use rand::Rng;

enum State {
	Closed,
	Open
}

struct Door {
	state: State,
	has_prize: bool,
	is_selected: bool
}

impl Door {
	fn new() -> Door {
		Door {
			state: State::Closed,
			has_prize: false,
			is_selected: false
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
	let mut doors: [Door; 3] = [Door::new(), Door::new(), Door::new()];
	
	let mut choice: usize = 0;
	
	// put the prize in one door
	doors[select_door(doors.len())].has_prize = true;

	// Let the user choose their door
	loop {
		println!("Which door do you choose? (1, 2, 3)");
		read_input(&mut choice);
		
		if check_if_valid(&choice, &[1, 2, 3]) {
			break;
		}
		println!("Invalid number entered!");
	}

	// Open 1 door
	monty_open_door(&mut doors);

	loop {
		println!("Do you want to change the door? (yes = 1, no = 0)");
		read_input(&mut choice);
		
		if check_if_valid(&choice, &[0, 1]) {
			break;
		}
		println!("Invalid number entered!");
	}
}

fn read_input(data: &mut usize) {
	let mut raw_data = String::new();
	io::stdin().read_line(&mut raw_data).expect("Error while reading from stdin!");

	*data = raw_data.trim()
				.parse()
				.expect("Unable to parse inputed data!");
}

fn check_if_valid(data: &usize, range: &[usize]) -> bool {
	let mut out: bool = false;

	for element in range {
		if *data == *element {
			out = true;
		}
	}
	out
}

fn select_door(max_index: usize) -> usize {
	rand::thread_rng().gen_range(0..max_index) as usize
}

fn monty_open_door(doors: &mut [Door]) {
	loop {
		let index = select_door(doors.len());
		
		if !doors[index].is_open() && !doors[index].has_prize {
			doors[index].open();
			break;
		}
	}
}