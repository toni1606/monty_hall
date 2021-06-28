use monty_hall::data_structures::contestant::Contestant;

use monty_hall::game::user_data::{read_input, check_if_valid};
use monty_hall::game::game_modes::{game, simulation};

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