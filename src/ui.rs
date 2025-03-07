use ratatui::{prelude::*, widgets::*};
use std::fs;
use std::io;

fn get_conflict_content(file: &str) -> String {
    fs::read_to_string(file).unwrap_or_else(|_| "Failed to read file".to_string())
}

pub fn draw_ui(conflicts: &Vec<String>, selected: usize) -> io::Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        let size = frame.size();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(70), // Main content area
                Constraint::Percentage(30), // Bottom section
            ].as_ref())
            .split(size);

        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30), // File list
                Constraint::Percentage(70), // Conflict details
            ].as_ref())
            .split(layout[0]);

        let right_layout = layout[0];
        let bottom_layout = layout[1];

        // Left pane - Merge Conflicts List
        let file_list_block = Block::default().borders(Borders::ALL).title("Merge Conflicts");
        let items: Vec<ListItem> = conflicts.iter().map(|c| ListItem::new(c.clone())).collect();
        let list = List::new(items).block(file_list_block).highlight_symbol("➜");
        let mut state = ListState::default();
        state.select(Some(selected));
        frame.render_stateful_widget(list, left_layout[0], &mut state);

        // Left pane - Conflict Details
        let conflict_content = get_conflict_content(&conflicts[selected]);
        let details_block = Block::default().borders(Borders::ALL).title("Conflict Details");
        let details = Paragraph::new(conflict_content).block(details_block).wrap(Wrap { trim: false });
        frame.render_widget(details, left_layout[1]);

        // Right pane - Main Content
        let content_block = Block::default().borders(Borders::ALL).title("File Content");
        let content_paragraph = Paragraph::new("(Full file content here)").block(content_block).wrap(Wrap { trim: false });
        frame.render_widget(content_paragraph, right_layout);

        // Bottom pane - Instructions
        let instructions_block = Block::default().borders(Borders::ALL).title("Instructions");
        let instructions = Paragraph::new("Use Arrow Keys to Navigate\nPress ESC to Exit")
            .block(instructions_block)
            .wrap(Wrap { trim: false });
        frame.render_widget(instructions, bottom_layout);
    })?;

    Ok(())
}
