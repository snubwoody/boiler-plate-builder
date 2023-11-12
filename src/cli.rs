use std::time::Duration;
use crossterm::{
    event::{self,KeyCode::{*, self},read,Event,KeyEvent, KeyEventKind},
    terminal::{
		disable_raw_mode, 
		enable_raw_mode, 
		EnterAlternateScreen, 
		LeaveAlternateScreen
	},
    ExecutableCommand
};
use ratatui::{
    prelude::*,
    widgets::*
};
use std::io::{stdout, Result};


pub fn cli() -> Result<()>{
	stdout().execute(EnterAlternateScreen)?;
	enable_raw_mode()?;
	let backend = CrosstermBackend::new(stdout());
	let mut terminal = Terminal::new(backend)?;
	terminal.clear()?;

	let header_text = "hello";

	loop {
		terminal.draw(|frame|{
			let layout = Layout::default()
				.direction(Direction::Vertical)
				.constraints(vec![
					Constraint::Percentage(5),
					Constraint::Percentage(95),
				])
				.split(frame.size());	

			let title = layout[0];
			let boqdy = layout[1];

			let body_layout = Layout::default()
				.direction(Direction::Vertical)
				.constraints(vec![
					Constraint::Percentage(10),
					Constraint::Percentage(80),
				])
				.split(body);

			let header = body_layout[0];
			let options = body_layout[1];


			frame.render_widget(
				Paragraph::new("Boilerplate builder (press 'q' to quit)").white(),
				title
			);
			frame.render_widget(
				Paragraph::new(header_text).white(),
				header
			);

			let items = [ListItem::new("React"),ListItem::new("Vue"),ListItem::new("Angular")];	
			
			frame.render_widget(
				List::new(items)
				.block(Block::default().title("What framework are you using?").borders(Borders::ALL))
    			.style(Style::default().fg(Color::White))
    			.highlight_style(Style::default())
    			.highlight_symbol(">>"),
				options
			);
		})?;

		match  get_input()? {
			Char('q') => {
				break;
			}
			_=>{}
		}

	}

	stdout().execute(LeaveAlternateScreen)?;
	disable_raw_mode()?;

	Ok(())
}

fn get_input() -> Result<KeyCode>{
	if event::poll(Duration::from_secs(4))? {
		match read()? {
			Event::Key(KeyEvent{code,kind,..}) => {
				if kind == KeyEventKind::Press{
					match code {
						Enter => {
							Ok(Enter)
						},
						Char('w') => {
							Ok(Char('w'))
						},
						Char('q') => {
							Ok(Char('q'))
						},
						_=>{Ok(Esc)}
					}
				}
				else {
					Ok(Esc)
				}
			}
			_=>{Ok(Esc)}
		}
	}
	else {
		Ok(Esc)
	}
}
struct Options<'a>{
	selected: bool,
	options: Vec<&'a str> 
}
