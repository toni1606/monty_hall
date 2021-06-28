use crate::game::state::State;

pub struct Door {
	state: State,
	has_prize: bool,
	is_selected: bool
}

impl Door {
	pub fn new() -> Door {
		Door {
			state: State::Closed,
			has_prize: false,
			is_selected: false
		}
	}

	pub fn get_is_selected(self) -> bool {
		self.is_selected
	}

	pub fn set_is_selected(&mut self, value: &bool) {
		self.is_selected = *value;
	}
	
	pub fn get_has_prize(self) -> bool {
		self.has_prize
	}
	
	pub fn is_open(&self) -> bool {
		match self.state {
			State::Open		=> true,
			State::Closed	=> false
		}
	}
	
	pub fn open(&mut self) {
		self.state = State::Open;
	}
	
}