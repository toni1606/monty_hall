pub struct Contestant {
	choice: usize,
	changes_door: bool
}

impl Contestant {
	pub fn new_alice() -> Contestant {
		Contestant {
			choice: 1,
			changes_door: false
		}
	}

	pub fn new_bob() -> Contestant {
		Contestant {
			choice: 1,
			changes_door: true
		}
	}

	pub fn get_choice(self) -> usize {
		self.choice
	}

	pub fn does_change_door(self) -> bool {
		self.changes_door
	}
}