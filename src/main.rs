mod utils;
mod projects;

use crate::{utils::clear, projects::{game, calculator}};

const GAME: &str = "0";
const CALCULATOR: &str = "1";

fn main() {
	clear::by_os();

	println!("Hello, Platzi!");
	println!();
	println!("Here exist two projects, choose one of them:");
	println!("[0] Game");
	println!("[1] Calculator");

	let mut option = String::new();

	std::io::stdin().read_line(&mut option).unwrap();

	let option = option.trim().to_string();

	clear::by_os();

	if option == GAME.to_string() {
		println!("Welcome to the game");
		println!();
		game::start();
	} else if option == CALCULATOR.to_string() {
		println!("Welcome to calculator");
		println!();
		calculator::start();
	}
}
