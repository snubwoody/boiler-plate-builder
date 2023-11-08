
use std::io::{Read,self, Error};
use std::fs::{File,create_dir, self};
use std::env;
use std::path::{PathBuf, Path};
use boilerplate_builder::Component::{*,self};
use boilerplate_builder::file_to_string;
use colored::*;
use std::process::Command;
use console::Term;


fn main() {
	//TODO make sure to make items responsive
	let args: Vec<_> = env::args().collect();

	let root_dir = PathBuf::from("./next-app");
	let src_dir = root_dir.join("src");
	let app_dir = src_dir.join("app");

	//TODO global.css not clearing as intended
	
	let _k: fs::ReadDir;

	match fs::read_dir(src_dir.join("components")) {
		Ok(files) => {
			_k = files
		}
		Err(err) => {

			if(err.raw_os_error().unwrap() == 3) {
				create_dir(src_dir.join("components")).unwrap()
			}
						
		}
	}

	println!("What would you like to do?");

	let options = vec![
		Option::new(true, "Generate component"),
		Option::new(false, "Generate route"),
	];

	options.iter().for_each(|option| option.print());


	println!("still here")
}



fn generate_component(component:Component,src_dir:PathBuf){
	match component {
		navbar => {
			fs::write(
				src_dir.join("components/navbar.tsx"),
				file_to_string(PathBuf::from("src/navbar.txt")).unwrap()
			).unwrap();
		},
		button => {
			fs::write(
				src_dir.join("components/button.tsx"),
				file_to_string(PathBuf::from("src/button.txt")).unwrap()
			).unwrap();
		}
	}
}

fn generate_route(app_dir:PathBuf,route_name:&str) -> Result<&str,io::Error>{
	match create_dir(app_dir.join(route_name)) {
		Ok(_) => {
			let route_dir = app_dir.join(route_name);
			fs::write(route_dir.join("page.tsx"), "").unwrap();
			return Ok("success");
		}
		Err(err) => {
			return Err(err)
		}
	}
	
	
}

struct Option<'a>{
	selected:bool,
	text:&'a str
}

impl<'a> Option<'a> {

	fn new(selected:bool,text:&'a str) -> Self {
		Option { selected, text }
	}

	fn print(&self) {
		if self.selected {
			print!("[{}] ",self.text.blue().underline());
		}
		else {
			print!{"[{}] ",self.text};
		}	
	}
}