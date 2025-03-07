mod logic;
mod ui;

use crossterm::{event::{self, KeyCode}, terminal};
use logic::get_conflicted_files;
use ui::draw_ui;
use std::io;

fn main() -> io::Result<()> {
    let conflicts = get_conflicted_files()?;

    if conflicts.is_empty() {
        println!("No merge conflicts detected.");
        return Ok(());
    }

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, terminal::EnterAlternateScreen)?;

    let mut selected = 0;
    loop {
        draw_ui(&conflicts, selected)?;

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Up => if selected > 0 { selected -= 1; },
                KeyCode::Down => if selected < conflicts.len() - 1 { selected += 1; },
                _ => {}
            }
        }
    }

    crossterm::execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

