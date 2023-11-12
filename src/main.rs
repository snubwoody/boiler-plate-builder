use std::future::Future;
use std::io::{self, Error};
use std::fs::{create_dir, self};
use std::env;
use std::path::{PathBuf, Path};
use boilerplate_builder::Component::{*,self};
use boilerplate_builder::file_to_string;
use std::{time::Duration};
use crossterm::event::{poll, read, Event,KeyCode};
mod cli;
use cli::cli;



fn main() {
	//TODO make sure to make items responsive
	let args: Vec<_> = env::args().collect();

	let root_dir = PathBuf::from("./next-app");
	let src_dir = root_dir.join("src");
	let app_dir = src_dir.join("app");

	//TODO global.css not clearing as intended
	init(src_dir);

	//cli();
	cli().unwrap();
	

	
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

