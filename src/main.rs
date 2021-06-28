
use rand::Rng;

use monty_hall::data_structures::door::Door;
use monty_hall::data_structures::contestant::Contestant;
use monty_hall::game::user_data::*;

fn main() {
	loop {
		let mut choice: usize = 0;
		println!("Do you want to run the simulation or play the game? (sim = 0, play = 1)");
		
		read_input(&mut choice);

		if check_if_valid(&choice, &[0, 1]) {
			if choice == 0 {
				let sim = simulation(&[Contestant::new_alice(), Contestant::new_bob()], &1000);

				println!("Alice: {:.3}%\nBob: {:.3}%", sim[0] * 100f64, sim[1] * 100f64);
			} else {
				game();
			}

			break;
		}

		println!("Invalid number entered! Retry...");
	}
}

fn select_door(max_index: usize) -> usize {
	rand::thread_rng().gen_range(0..max_index) as usize
}

fn monty_open_door(doors: &mut [Door]) {
	loop {
		let index = select_door(doors.len());
		
		if !doors[index].is_open() && !doors[index].get_has_prize() && !doors[index].get_is_selected() {
			doors[index].open();
			break;
		}
	}
}

fn change_door(doors: &mut [Door]) {
	for door in doors {
		if door.get_is_selected() {
			door.set_is_selected(false);
		} else if !door.get_is_selected() && !door.is_open() {
			door.set_is_selected(true);
		}
	}
}

fn chek_if_won(doors: &mut [Door]) -> bool {
	let mut out = false;
	for door in doors {
		if door.get_is_selected() && door.get_has_prize() {
			out = true;
		}
	}
	out
}

fn game() {
	let mut doors: [Door; 3] = [Door::new(), Door::new(), Door::new()];
	
	let mut choice: usize = 0;
	
	// put the prize in one door
	doors[select_door(doors.len())].set_has_prize(true);

	// Let the user choose their door
	loop {
		println!("Which door do you choose? (1, 2, 3)");
		read_input(&mut choice);
		
		if check_if_valid(&choice, &[1, 2, 3]) {
			break;
		}
		println!("Invalid number entered!");
	}
	doors[choice - 1].set_is_selected(true);

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

	if choice == 1 {
		change_door(&mut doors);
	}

	if chek_if_won(&mut doors) {
		println!("Congrats! You won the prize!!! 🎉");
	} else {
		println!("Sorry, you lost! 😢")
	}
}

fn simulation(contestants: &[Contestant], simulate_contestant: &usize) -> Box<[f64]> {
	let mut simulation_output = vec![0.0f64; contestants.len()];
	
	let mut i = 0;
	for contestant in contestants {
		let mut won: usize = 0;
		for _i in 0..*simulate_contestant {
			let mut doors: [Door; 3] = [Door::new(), Door::new(), Door::new()];

			// put the prize in one door
			doors[select_door(doors.len())].set_has_prize(true);

			// Let the user choose their door
			doors[contestant.get_choice() - 1].set_is_selected(true);

			// Open 1 door
			monty_open_door(&mut doors);

			// Change door if contestant chooses to change
			if contestant.does_change_door() {
				change_door(&mut doors);
			}

			if chek_if_won(&mut doors) {
				won += 1;
			}
		}
		simulation_output[i] = won as f64 / *simulate_contestant as f64;
		i += 1;
	}

	simulation_output.into_boxed_slice()
}