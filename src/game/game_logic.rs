use rand::Rng;
use crate::data_structures::door::Door;

pub fn select_door(max_index: usize) -> usize {
	rand::thread_rng().gen_range(0..max_index) as usize
}

pub fn monty_open_door(doors: &mut [Door]) {
	loop {
		let index = select_door(doors.len());
		
		if !doors[index].is_open() && !doors[index].get_has_prize() && !doors[index].get_is_selected() {
			doors[index].open();
			break;
		}
	}
}

pub fn change_door(doors: &mut [Door]) {
	for door in doors {
		if door.get_is_selected() {
			door.set_is_selected(false);
		} else if !door.get_is_selected() && !door.is_open() {
			door.set_is_selected(true);
		}
	}
}

pub fn chek_if_won(doors: &mut [Door]) -> bool {
	let mut out = false;
	for door in doors {
		if door.get_is_selected() && door.get_has_prize() {
			out = true;
		}
	}
	out
}