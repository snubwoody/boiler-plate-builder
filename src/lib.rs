use std::io::{Read,self};
use std::fs::File;
use std::fs::{create_dir, self};
use std::path::{PathBuf, Path};
use Component::*;

pub enum Component {
	navbar,
	button
}

pub enum Page {
	home,
	signin,
	catalog,
	profile
}

pub fn file_to_string(path:PathBuf) -> Result<String, io::Error>{
	//TODO handle the error
	let mut file_contents: String = String::new();
	let mut file: File = File::open(path)?;
	file.read_to_string(&mut file_contents)?;
	return Ok(file_contents);
}

pub fn generate_component(component:Component,src_dir:PathBuf){
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

pub fn generate_route<'a>(app_dir:&PathBuf,route_name:&str) -> Result<&'a str,io::Error>{
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
