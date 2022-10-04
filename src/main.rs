use std::process::exit;

fn main() {
	let input = r#"
include "../include/script.evs";
#asm
	include "defines.inc"
	include "dungeon.inc"
#end

script fn xGenerateScraper() {
	u8 x = 32;
	u8 y = 32;

	repeat 255 {
		step_direction(rand() & 3, x, y);
		map_put_tile(x, y, TILE_CLEAR);
	}

	map_put_tile(x, y, TILE_EXIT);
}

script fn xGenerateHalls() {
	u8 direction = rand() & 3;

	u8 width = rand() & 3 + 4;
	u8 height = rand() & 3 + 4;

	u8 x = 32 - (rand() & 3);
	u8 y = 32 - (rand() & 3);
	
	repeat height {
		repeat width {
			map_put_tile(x, y, TILE_CLEAR);
			x += 1;
		}
		x -= width;
		y += 1;
	}

	repeat 9 {
		repeat rand() & 1 + 2 {
			repeat rand() & 7 + 4 {
				map_put_tile(x, y, TILE_CLEAR);
				step_direction(direction, x, y);
			}

			direction += (rand() & 2) - 1 & 3;
		}

		u8 old_x = x;
		u8 old_y = y;

		width = (rand() & 3) + 3;
		height = (rand() & 3) + 3;
		
		repeat height {
			repeat width {
				map_put_tile(x, y, TILE_CLEAR);
				x += 1;
			}
			x -= width;
			y += 1;
		}

		x = old_x;
		y = old_y;

		if direction == UP {
			y -= 1;
		} else if direction == LEFT {
			x += width;
		} else if direction == DOWN {
			x += width - 1;
			y += height;
		} else if direction == RIGHT {
			x -= 1;
			y += height - 1;
		}
	}

	width = 3;
	height = 3;
	x -= 1;
	y -= 1;

	repeat height {
		repeat width {
			map_put_tile(x, y, TILE_CLEAR);
			x += 1;
		}
		x -= width;
		y += 1;
	}
	x += 1;
	y -= 2;
	map_put_tile(x, y, TILE_EXIT);
}

script fn xGenerateItems() {
	do {
		u8 x = rand() & 63;
		u8 y = rand() & 63;
		u8 tile = map_get_tile(x, y);
	} while tile != TILE_CLEAR;

	// The items are given a weight in the following order:
	// ITEM0 - 6/16, 37.50% chance
	// ITEM1 - 6/16, 37.50% chance
	// ITEM2 - 3/16, 18.75% chance
	// ITEM3 - 1/16,  6.25% chance

	// Clamp the random number to (0, 15) and choose an item.
	u8 item_id = rand() & 15;

	if item_id < 6 { map_put_tile(x, y, TILE_ITEM0); }
	else if item_id < 12 { map_put_tile(x, y, TILE_ITEM1); }
	else if item_id < 15 { map_put_tile(x, y, TILE_ITEM2); }
	else { map_put_tile(x, y, TILE_ITEM3); }
}
"#;
	match evscript::parse(input) {
		Ok(ast) => println!("{ast:#?}"),
		Err(err) => {
			eprintln!("<test>: {err}");
			exit(1);
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn example_header() {
		let input = r#"
#asm include "script.inc" #end

env script {
	use std;
	def memset(ptr, u8, u16);
	def rand(u8);
	def print(ptr);
	def say(ptr);
	def print_wait();
	def get_flag(u8, u8);
	def map_put_tile(u8, u8, u8);
	def map_get_tile(u8, u8, u8);
	def step_direction(u8, u8, u8);
	def draw_sprite(u8);
	def npc_move(u8, u16, u16);
	def npc_set_frame(u8, u8);
	def npc_lock();

	pool = 16;
	terminator = return;
}

env npc {
	use script;
	macro move(...) = npc_move;
	macro set_frame(...) = npc_set_frame;
	alias lock() = npc_lock();
	alias wait() = print_wait();
	pool = 8;
}
"#;

		if let Err(err) = evscript::parse(input) {
			panic!("<example_header>: {err}");
		}
	}

	#[test]
	fn example_script() {
		let input = r#"
include "../include/script.evs";
#asm
	include "res/charmap.inc"
	include "entity.inc"
#end

npc fn xWalkAround() {
	lock();
	say("Hello, World!<WAITBTN>");
	wait();
	set_frame("self", ENTITY_FRAME_STEP);
	repeat 60 {
		move("self", 12, 0);
		yield;
	}
	set_frame("self", ENTITY_FRAME_IDLE);
	repeat 60 {
		yield;
	}
	set_frame("self", ENTITY_FRAME_STEP);
	repeat 60 {
		move("self", -12, 0);
		yield;
	}
	set_frame("self", ENTITY_FRAME_IDLE);
	say("Bye!<WAITBTN><CLEAR>");
	wait();
	lock();
}
"#;

		if let Err(err) = evscript::parse(input) {
			panic!("<example_script>: {err}");
		}
	}

	#[test]
	fn example_function() {
		let input = r#"
include "../include/script.evs";
#asm
	include "defines.inc"
	include "dungeon.inc"
#end

script fn xGenerateHalls() {
	u8 direction = rand() & 3;

	u8 width = rand() & 3 + 4;
	u8 height = rand() & 3 + 4;

	u8 x = 32 - (rand() & 3);
	u8 y = 32 - (rand() & 3);
	
	repeat height {
		repeat width {
			map_put_tile(x, y, TILE_CLEAR);
			x += 1;
		}
		x -= width;
		y += 1;
	}

	repeat 9 {
		repeat rand() & 1 + 2 {
			repeat rand() & 7 + 4 {
				map_put_tile(x, y, TILE_CLEAR);
				step_direction(direction, x, y);
			}

			direction += (rand() & 2) - 1 & 3;
		}

		u8 old_x = x;
		u8 old_y = y;

		width = (rand() & 3) + 3;
		height = (rand() & 3) + 3;
		
		repeat height {
			repeat width {
				map_put_tile(x, y, TILE_CLEAR);
				x += 1;
			}
			x -= width;
			y += 1;
		}

		x = old_x;
		y = old_y;

		if direction == UP {
			y -= 1;
		} else if direction == LEFT {
			x += width;
		} else if direction == DOWN {
			x += width - 1;
			y += height;
		} else if direction == RIGHT {
			x -= 1;
			y += height - 1;
		}
	}

	width = 3;
	height = 3;
	x -= 1;
	y -= 1;

	repeat height {
		repeat width {
			map_put_tile(x, y, TILE_CLEAR);
			x += 1;
		}
		x -= width;
		y += 1;
	}
	x += 1;
	y -= 2;
	map_put_tile(x, y, TILE_EXIT);
}
"#;

		if let Err(err) = evscript::parse(input) {
			panic!("<example_script>: {err}");
		}
	}
}