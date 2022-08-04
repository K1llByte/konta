use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::state::*;

pub enum Event {
    Input(KeyEvent),
    Tick,
}

pub fn items_input_handler(input: &Event, app: &mut AppState) -> bool {
    if let FocusedWindow::Items(idx) = &mut app.focused {
        match input {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
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
                // KeyCode::Tab => {
                //     app.focused = FocusedWindow::People(0);
                // },
                // Select Owner of this item purchase
                KeyCode::Enter => {
                    app.focused = FocusedWindow::OwnerSelector(*idx,0);
                }
                // Add a new person
                KeyCode::Char('a') | KeyCode::Char('A') => {
                    // TODO:
                    // Change Focused window to AddPerson
                    app.focused = FocusedWindow::AddPerson(String::with_capacity(30));
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
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    return true;
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.data.people.len() > 0 && *idx < app.data.people.len() - 1 {
                        *idx += 1;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if *idx > 0usize {
                        *idx -= 1;
                    }
                }
                // KeyCode::Tab => {
                //     app.focused = FocusedWindow::Items(0);
                // }
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
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    return true;
                }
                KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                    if app.data.people.len() > 0 && *person_idx < app.data.people.len() - 1 {
                        *person_idx += 1;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                    if *person_idx > 0usize {
                        *person_idx -= 1;
                    }
                }
                KeyCode::Enter => {
                    if *person_idx < app.data.people.len() {
                        app.data.set_item_owner(*item_idx, Some(*person_idx));
                    }
                    app.focused = FocusedWindow::Items(*item_idx);
                }
                KeyCode::Esc => {
                    app.focused = FocusedWindow::Items(*item_idx);
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }
    return false;
}

pub fn add_person_input_handler(event: &Event, app: &mut AppState) -> bool {
    if let FocusedWindow::AddPerson(name) = &mut app.focused {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char(c) => {
                    name.push(c);
                }
                KeyCode::Backspace => {
                    use substring::Substring;
                    if name.len() > 0 {
                        // if event.modifiers.contains(KeyModifiers::CONTROL) {
                        //     name.clear();
                        // }
                        // else {
                            *name = String::from(name.substring(0, name.len()-1))
                        // }
                    }
                }
                KeyCode::Enter => {
                    // if name.len() > 0 {
                        app.data.people.push(name.clone());
                        app.focused = FocusedWindow::Items(0);
                    // }
                }
                KeyCode::Esc => {
                    app.focused = FocusedWindow::Items(0);
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }
    return false;
}