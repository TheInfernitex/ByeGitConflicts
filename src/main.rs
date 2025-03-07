use git2::Repository;
use ratatui::{prelude::*, widgets::*};
use crossterm::{event::{self, KeyCode}, terminal};
use std::io;

fn main() -> io::Result<()> {
    let repo = Repository::open(".").expect("Failed to open repository");
    let statuses = repo.statuses(None).expect("Failed to get repository status");
    
    let conflicts: Vec<String> = statuses.iter()
        .filter(|entry| entry.status().is_conflicted())
        .filter_map(|entry| entry.path().map(String::from))
        .collect();
    
    if conflicts.is_empty() {
        println!("No merge conflicts detected.");
        return Ok(());
    }
    
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, terminal::EnterAlternateScreen)?;
    
    let mut selected = 0;
    loop {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            let size = frame.size();
            let block = Block::default().borders(Borders::ALL).title("Merge Conflicts");
            let items: Vec<ListItem> = conflicts.iter().map(|c| ListItem::new(c.clone())).collect();
            let list = List::new(items).block(block).highlight_symbol("➜");
            
            let mut state = ListState::default();
            state.select(Some(selected));
            frame.render_stateful_widget(list, size, &mut state);
        })?;
        
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
