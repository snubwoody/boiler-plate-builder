use std::io::{Read,self};
use std::fs::File;
use std::path::PathBuf;

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