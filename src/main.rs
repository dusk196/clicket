use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io;

// The ASCII Art (Using Rust's raw string syntax r#...#)
const BANNER: &str = r#"
 ██████╗ ██╗      ██╗         ██████╗ ██╗  ██╗ ███████╗ ████████╗
██╔════╝ ██║      ██║        ██╔════╝ ██║ ██╔╝ ██╔════╝ ╚══██╔══╝
██║      ██║      ██║ █████╗ ██║      █████╔╝  █████╗      ██║   
██║      ██║      ██║ ╚════╝ ██║      ██╔═██╗  ██╔══╝      ██║   
╚██████╗ ███████╗ ██║        ╚██████╗ ██║  ██╗ ███████╗    ██║   
 ╚═════╝ ╚══════╝ ╚═╝         ╚═════╝ ╚═╝  ╚═╝ ╚══════╝    ╚═╝   
                                                                 
                        >_ Live Cricket Scores from your Terminal
"#;

fn main() -> Result<()> {
    // 1. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Run the Application Loop
    let res = run_app(&mut terminal);

    // 3. Restore Terminal (CRITICAL step or your terminal will be broken on exit)
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    loop {
        terminal.draw(|f| {
            // Create a layout area
            let size = f.area();
            
            // Create the Banner Widget
            let paragraph = Paragraph::new(BANNER)
                .block(Block::default().borders(Borders::ALL).title(" CLI-cket "))
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Center);

            // Render it in the center of the screen
            // (We keep it simple and use the full screen for now)
            f.render_widget(paragraph, size);
        })?;

        // Handle Input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}