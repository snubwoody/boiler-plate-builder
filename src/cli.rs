use std::io::{self, Error};
use std::ops::Index;
use std::time::Duration;
use colored::*;
use crossterm::event::KeyEvent;
use crossterm::{
    event::{self,KeyCode::*, KeyEventKind,poll,read,Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, 
	LeaveAlternateScreen,Clear,ClearType},
    ExecutableCommand,
	execute
};
use ratatui::{
    prelude::*,
    widgets::Paragraph
};

use std::io::{stdout, Result};


pub fn cli() -> Result<()>{
	stdout().execute(EnterAlternateScreen)?;
	enable_raw_mode()?;
	let backend = CrosstermBackend::new(stdout());
	let mut terminal = Terminal::new(backend)?;
	terminal.clear()?;

	let options = vec!["[Create route]","[Create component]"];
	let mut selected = 0;

	loop {
		terminal.draw(|frame|{
			let area = frame.size();
			

			let layout = Layout::default()
				.direction(Direction::Vertical)
				.constraints(vec![
					Constraint::Percentage(5),
					Constraint::Percentage(95),
				])
				.split(frame.size());	

			let header = layout[0];
			let body = layout[1];

			frame.render_widget(
				Paragraph::new("Boilerplate builder (press 'q' to quit)").white(),
				header
			);
			frame.render_widget(
				Paragraph::new("Hello this is a test\nYou like it?").white(),
				body
			);		
		})?;

		if event::poll(Duration::from_secs(4))? {
			match read()? {
				Event::Key(KeyEvent{code,..}) => match code {
					Enter => {
					},
					Char('q') => {
						break;
					}
					_=>{}
					}

				_=>{}
			}
		}
	}

	stdout().execute(LeaveAlternateScreen)?;
	disable_raw_mode()?;

	

	Ok(())
}

struct Options<'a>{
	selected: bool,
	options: Vec<&'a str> 
}
