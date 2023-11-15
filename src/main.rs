use std::io::{self, stdout, Write};
use std::fs::{create_dir, self};
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use crossterm::execute;
use crossterm::{
    event::{self,KeyCode::*,read,Event,KeyEvent, KeyEventKind},
    terminal::{
		disable_raw_mode, 
		enable_raw_mode, 
		EnterAlternateScreen, 
		LeaveAlternateScreen
	},
    ExecutableCommand
};
use boilerplate_builder::generate_route;
mod cli;




fn main() {
	//TODO make sure to make items responsive
	let _args: Vec<_> = env::args().collect();

	let root_dir = PathBuf::from("./next-app");
	let src_dir = root_dir.join("src");
	let app_dir = src_dir.join("app");

	let clear_scr = "\x1B[2J";
	let reset_csr_pos = "\x1B[H";
	//TODO global.css not clearing as intended
	init(src_dir);

	execute!(stdout(),EnterAlternateScreen).unwrap();
	enable_raw_mode().unwrap();


	let mut options = OptionsList::new(vec!["Genarate Component","Generate Route"],0);

	loop {
		print!("{}",clear_scr);
		print!("{}",reset_csr_pos);

		println!("{}",options.selected);
		
		options.print();

		if event::poll(Duration::from_secs(4)).unwrap() {
			match read().unwrap() {
				Event::Key(KeyEvent{code,kind,..}) => {
					if kind == KeyEventKind::Press{
						match code {
							Char('q') => {
								break;
							},
							Up => {
								options.selected +=1;
							}
							Down => {
								options.selected -=1;
							}
							Enter => {
								print!("{}",clear_scr);
								generate_route(app_dir,"signup").unwrap();
								println!("Generated route 'Signup' in app directory");
								break;
							}
							_=>{}
						}
					}
					
				}
				_=>{}
			}
		}

		io::stdout().flush().unwrap();
	}

	disable_raw_mode().unwrap();
	execute!(stdout(),LeaveAlternateScreen).unwrap();
}

fn init(src_dir:PathBuf){
	match fs::read_dir(src_dir.join("components")) {
		Ok(_files) => {}
		Err(err) => {

			if err.raw_os_error().unwrap() == 3 {
				create_dir(src_dir.join("components")).unwrap()
			}
						
		}
	}
}

struct OptionsList<'a>{
	text: Vec<&'a str>,
	selected: usize
}

impl<'a> OptionsList<'a> {
	fn new(text:Vec<&'a str>,selected:usize) -> Self{
		OptionsList { text, selected }
	}

	fn print(&self) {
		for (index,option) in self.text.iter().enumerate() {
			if index == self.selected {
				println!("\x1B[1;35m {} \x1B[0m",option)
			}
			else {
				println!("{}",option)
			}
		}
	}
}




