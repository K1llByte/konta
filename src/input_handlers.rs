use crossterm::event::{KeyCode, KeyEvent};

use crate::state::*;

pub enum Event {
    Input(KeyEvent),
    Tick,
}

pub fn items_input_handler(input: &Event, app: &mut AppState) -> bool {
    if let FocusedWindow::Items(idx) = &mut app.focused {
        match input {
            Event::Input(event) => match event.code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    return true;
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if *idx < app.data.items.len() - 1 {
                        *idx += 1;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if *idx > 0usize {
                        *idx -= 1;
                    }
                }
                KeyCode::Tab => {
                    app.focused = FocusedWindow::People(0);
                },
                KeyCode::Enter => {
                    app.focused = FocusedWindow::OwnerSelector(*idx,0);
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }
    return false;
}

pub fn people_input_handler(event: &Event, app: &mut AppState) -> bool {
    if let FocusedWindow::People(idx) = &mut app.focused {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    return true;
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if *idx < app.data.people.len() - 1 {
                        *idx += 1;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if *idx > 0usize {
                        *idx -= 1;
                    }
                }
                KeyCode::Tab => {
                    app.focused = FocusedWindow::Items(0);
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }
    return false;
}


pub fn owner_selector_input_handler(event: &Event, app: &mut AppState) -> bool {
    if let FocusedWindow::OwnerSelector(item_idx, person_idx) = &mut app.focused {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    return true;
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if *person_idx < app.data.people.len() - 1 {
                        *person_idx += 1;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if *person_idx > 0usize {
                        *person_idx -= 1;
                    }
                }
                KeyCode::Enter => {
                    app.data.set_item_owner(*item_idx, Some(*person_idx));
                    app.focused = FocusedWindow::Items(*item_idx);
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }
    return false;
}
