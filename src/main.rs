use std::io::{self, stdout, Write};
use std::fs::{create_dir, self};
use std::env;
use std::path::PathBuf;
use crossterm::{
	execute,
	terminal::{EnterAlternateScreen,LeaveAlternateScreen},
	event::{self,KeyCode::{*, self},read,Event,KeyEvent, KeyEventKind}
};
mod cli;
use cli::get_input;



fn main() {
	//TODO make sure to make items responsive
	let args: Vec<_> = env::args().collect();

	let root_dir = PathBuf::from("./next-app");
	let src_dir = root_dir.join("src");
	let app_dir = src_dir.join("app");

	//TODO global.css not clearing as intended
	init(src_dir);

	//cli();
	//cli().unwrap();

	execute!(stdout(),EnterAlternateScreen).unwrap();

	let options = vec!["Generate Component","Generate route"];

	loop {
		print!("\x1B[2J");
		print!("\x1B[H");
		print!("\x1B[1;32mhi");
		println!("Hi");
		
		options.iter().for_each(|t|println!("{}",t));
		match get_input().unwrap() {
			Char('q') => {
				break;
			}
			_=>{}
		}

		io::stdout().flush().unwrap();
	}

	execute!(stdout(),LeaveAlternateScreen).unwrap();
}

fn init(src_dir:PathBuf){
	match fs::read_dir(src_dir.join("components")) {
		Ok(files) => {}
		Err(err) => {

			if(err.raw_os_error().unwrap() == 3) {
				create_dir(src_dir.join("components")).unwrap()
			}
						
		}
	}
}




