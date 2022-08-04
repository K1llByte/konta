use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event as CEvent,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
    },
};
use std::{
    io,
    thread,
    time::Duration,
    sync::mpsc,
    time::{Instant},
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

/////////////////////////////////////

pub mod state;
use state::*;
pub mod ui;
use ui::*;
pub mod input_handlers;
use input_handlers::*;

/////////////////////////////////////


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup input Receiver thread
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
    
    // 2. Setup terminal UI renderer
    
    // Noncanonical mode, which eliminates the need to wait
    // for an Enter by the user to react to the input.
    enable_raw_mode()?;
    // setup terminal
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen/* , EnableMouseCapture */)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    terminal.clear()?;

    // App state
    let mut app = AppState::default();
    
    loop {
        // Draw
        terminal.draw(|f| {
            ui(f, &app);
        })?;

        
        // Input handlers
        let input = rx.recv()?;
        // Handle items input
        let exit = match app.focused {
            FocusedWindow::Items(_) => items_input_handler(&input, &mut app),
            FocusedWindow::People(_) => people_input_handler(&input, &mut app),
            FocusedWindow::OwnerSelector(_,_) => owner_selector_input_handler(&input, &mut app),
        };
        // Exit
        if exit {
            disable_raw_mode()?;
            terminal.show_cursor()?;
            break;
        }
    }


    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}