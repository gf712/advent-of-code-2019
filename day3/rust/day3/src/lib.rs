mod utils;

use std::fmt;
use std::cmp;
use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// PartialEq brings ==
// Copy needed for copying in loop
// Clone needed for Copy
#[wasm_bindgen]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Cell {
	Empty = 0,
	Occupied = 1,
	Intersection = 2
}

#[wasm_bindgen]
pub struct GridWorld {
	width: usize,
	height: usize,
	cells: Vec<Cell>
}

#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct GridPoint(usize, usize);

#[derive(Debug)]
enum Directions {
	Left = 0,
	Right = 1,
	Up = 2,
	Down = 3
}

#[wasm_bindgen]
impl GridWorld {
    pub fn new(width: usize, height: usize) -> GridWorld {

        let cells = (0..width * height)
            .map(|_| Cell::Empty)
            .collect();

        GridWorld {
            width,
            height,
            cells,
        }
    }

    pub fn parse(instructions: String) -> Result<GridWorld, JsValue> {
    	let instruction_vec = instructions.split(",");
    	let mut left = 0;
    	let mut right = 0;
    	let mut up = 0;
    	let mut down = 0;
    	let mut point = Point(0, 0);
    	let mut direction_vec = Vec::<Directions>::new();
    	let mut magnitude_vec = Vec::<usize>::new();

    	for instruction in instruction_vec {

    		let magnitude = match (&instruction[1..]).parse::<usize>() {
    			Ok(expr) => Ok(expr as i32),
    			Err(_) => Err(JsValue::from("Failed to parse instruction")),
    		}?;

    		match &instruction[..1] {
    			"R" => {
    				point.0 += magnitude;
    				right = cmp::max(right, point.0);
    				direction_vec.push(Directions::Right);
    			},
    			"L" => {
    				point.0 -= magnitude;
    				left = cmp::min(left, point.0);
    				direction_vec.push(Directions::Left);
    			},
    			"U" => {
    				point.1 += magnitude;
    				up = cmp::max(up, point.1);
    				direction_vec.push(Directions::Up);
    			},
    			"D" => {
    				point.1 -= magnitude;
    				down = cmp::min(down, point.1);
    				direction_vec.push(Directions::Down);
    			},
    			expr => return Err(JsValue::from(&format!("Unknown instruction {}", expr)))
    		};

    		magnitude_vec.push(magnitude as usize);
    	}
    	let width = (cmp::max(right, left.abs()) * 2 ) as usize;
    	let height = (cmp::max(up, down.abs()) * 2 ) as usize;

    	let width = if width % 2 == 0 {
    		width + 1
    	} else {
    		width
    	};

    	let height = if height % 2 == 0 {
    		height + 1
    	} else {
    		height
    	};

    	let mut result = GridWorld::new(width, height);
    	
    	let mut point = GridPoint(width / 2, height / 2);
    	result.cells[point.1 * result.width + point.0] = Cell::Occupied;

    	for (direction, magnitude) in direction_vec.iter().zip(magnitude_vec.iter()) {
    		match direction {
    			Directions::Left  => {
    				result.draw(point.0 - magnitude, point.0, point.1, point.1);
    				point.0 -= magnitude;
    			},
    			Directions::Right => {
    				result.draw(point.0, point.0 + magnitude, point.1, point.1);
    				point.0 += magnitude;
    			},
    			Directions::Up    => {
    				result.draw(point.0, point.0, point.1 - magnitude, point.1);
    				point.1 -= magnitude;
    			},
    			Directions::Down  => {
    				result.draw(point.0, point.0, point.1, point.1 + magnitude);
    				point.1 += magnitude;
    			},
    		};
    		result.cells[point.1 * result.width + point.0] = Cell::Occupied;
    	}

    	Ok(result)
    }

    pub fn draw(&mut self, x1: usize, x2: usize, y1: usize, y2: usize) -> () {
    	if x1 == x2 {
    		for el in self.cells[y1*self.width + x1.. y2*self.width + x2].iter_mut().step_by(self.width) {
    			*el = Cell::Occupied;
    		}
    	}
    	else if y1 == y2 {
    		for el in self.cells[y1*self.width + x1.. y2*self.width + x2].iter_mut() {
    			*el = Cell::Occupied;
    		}
    	}
    	else {
    		// diagonals are not possible
    	}
    }

    pub fn superimpose(&self, other: &GridWorld) -> GridWorld {
    	let max_width = cmp::max(self.width, other.width);
    	let max_height = cmp::max(self.height, other.height);

    	let mut result = GridWorld::new(max_width, max_height);

    	let origin_point = GridPoint(max_width / 2, max_height / 2);
    	let self_origin_point = GridPoint(self.width / 2, self.height / 2);
    	let other_origin_point = GridPoint(other.width / 2, other.height / 2);

    	let self_to_new_vec = Point(self_origin_point.0 as i32 - origin_point.0 as i32, self_origin_point.1 as i32 - origin_point.1 as i32);

		result.cells[origin_point.1 * result.width + origin_point.0] = Cell::Occupied;

		for (idx, cell) in self.cells.iter().enumerate() {
			if *cell == Cell::Occupied {
				let self_y = (idx / self.width) as i32;
				let self_x = (idx % self.width) as i32;
				result.cells[((self_y-self_to_new_vec.1) * result.width as i32 + self_x- self_to_new_vec.0) as usize] = Cell::Occupied;
			}
		}

    	let other_to_new_vec = Point(other_origin_point.0 as i32 - origin_point.0 as i32, other_origin_point.1 as i32 - origin_point.1 as i32);

		for (idx, cell) in other.cells.iter().enumerate() {
			if *cell == Cell::Occupied {
				let self_y = (idx / other.width) as i32;
				let self_x = (idx % other.width) as i32;
				let val = &mut(result.cells[((self_y-other_to_new_vec.1) * result.width as i32 + self_x- other_to_new_vec.0) as usize]);
				if *val == Cell::Occupied {
					*val = Cell::Intersection;
				} 
				else {
					*val = Cell::Occupied;
				}
			}
		}
    	result
    }

    pub fn compute_intersections(&self) -> usize {
    	let mut result = self.height + self.width + 1;
    	let mid_point = Point((self.width/2) as i32, (self.height/2) as i32);
    	for (idx, cell) in self.cells.iter().enumerate() {
    		if *cell == Cell::Intersection {
				let y = (idx / self.width) as i32;
				let x = (idx % self.width) as i32;
    			let dist = ((x-mid_point.0).abs()+(y-mid_point.1).abs()) as usize;
    			if dist < result && dist > 0 {
    				result = dist;
    			}
    		}
    	}
    	result
	}

	pub fn render(&self) -> String {
		self.to_string()
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height
	}

	pub fn cells(&self) -> *const Cell {
		self.cells.as_ptr()
	}
}

impl fmt::Display for GridWorld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
					Cell::Empty => '◻',
					Cell::Occupied => '◼',
					Cell::Intersection => 'x',
				};
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn generate_empty_world() {
    	let world = GridWorld::new(64, 64);
        assert_eq!(world.cells.len(), 64*64);
    }

    #[test]
    fn draw_horizontal_line() {
    	let mut world = GridWorld::new(64, 64);
    	world.draw(0, 0, 0, 10);
        assert_eq!(world.cells[0], Cell::Occupied);
    }

    #[test]
    fn compute_shortest_dist_1() {
    	let world1 = GridWorld::parse("R8,U5,L5,D3".to_string()).unwrap();
    	let world2 = GridWorld::parse("U7,R6,D4,L4".to_string()).unwrap();
    	let complete_world = world1.superimpose(&world2);
    	assert_eq!(complete_world.compute_intersections(), 6);
    }

    #[test]
    fn compute_shortest_dist_2() {
    	let world1 = GridWorld::parse("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string()).unwrap();
    	let world2 = GridWorld::parse("U62,R66,U55,R34,D71,R55,D58,R83".to_string()).unwrap();
    	let complete_world = world1.superimpose(&world2);
    	assert_eq!(complete_world.compute_intersections(), 159);
    }

    #[test]
    fn compute_shortest_dist_3() {
    	let world1 = GridWorld::parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string()).unwrap();
    	let world2 = GridWorld::parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()).unwrap();
    	let complete_world = world1.superimpose(&world2);
    	assert_eq!(complete_world.compute_intersections(), 135);
    }
}