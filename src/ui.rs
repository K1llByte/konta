use tui::{
    Frame,
    backend::Backend,
    layout::{
        Constraint, Direction, Layout
    },
    widgets::{
        Block, Borders,
        Cell, Row, Table, BorderType, List, ListItem
    },
    style::{
        Color, Modifier, Style
    },
    text::{
        Span
    },
};

use crate::state::*;


// Main UI render
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        // .margin(1)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.size());

    
    /////////////// Render people list /////////////

    let mut people_rows = Vec::with_capacity(app.data.people.len());
    for (i,person) in app.data.people.iter().enumerate() {
        // Get row selected or not
        let row = match app.focused {
            // Selected
            FocusedWindow::People(idx) | FocusedWindow::OwnerSelector(_,idx) if i == idx => 
                ListItem::new(person.as_ref())
                    .style(Style::default().bg(Color::White).fg(Color::Black)),
            // Normal
            _ => ListItem::new(person.as_ref()).style(Style::default().bg(person_color(i))/* .fg(Color::Black) */),
        };
        people_rows.push(row);
    }
    
    // Create List and customize layout
    let people_list = List::new(people_rows)
        .block(
            if let FocusedWindow::People(_) = app.focused {
                Block::default()
                    .borders(Borders::ALL)
                    .title("People")
                    .border_type(BorderType::Thick)
            }
            else {
                Block::default()
                    .borders(Borders::ALL)
                    .title("People")
            }
        );

    /////////////// Render items table /////////////
    let mut item_rows = Vec::with_capacity(app.data.items.len());

    for (i,item) in app.data.items.iter().enumerate() {
        // Get row selected or not
        let row = match app.focused {
            // Selected
            FocusedWindow::Items(idx) if i == idx => 
                Row::new(vec![
                        Cell::from(item.description.as_ref()),
                        Cell::from(item.quantity.to_string()),
                        Cell::from(item.price.to_string()),
                        Cell::from(owner_to_string(item.owner, &app))
                            .style(
                                if let Some(person_idx) = item.owner {
                                    Style::default().bg(person_color(person_idx)).fg(Color::White)
                                }
                                else {
                                    Style::default()
                                }
                            ),
                    ]
                )
                .style(Style::default().bg(Color::White).fg(Color::Black)),
            // Normal
            _ => Row::new(vec![
                    Cell::from(item.description.as_ref()),
                    Cell::from(item.quantity.to_string()),
                    Cell::from(item.price.to_string()),
                    Cell::from(owner_to_string(item.owner, &app))
                        .style(
                            if let Some(person_idx) = item.owner {
                                Style::default().bg(person_color(person_idx)).fg(Color::White)
                            }
                            else {
                                Style::default()
                            }
                        ),
                ]
            ),
        };
        item_rows.push(row);
    }

    // Create Table and customize layout
    let items_table = Table::new(item_rows)
    .block(
        if let FocusedWindow::Items(_) = app.focused {
            Block::default()
                .borders(Borders::ALL)
                .title("Items")
                .border_type(BorderType::Thick)
        }
        else {
            Block::default()
                .borders(Borders::ALL)
                .title("Items")
        }
    )
    .header(
        Row::new(vec![
            Cell::from(Span::styled(
                "Description",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Quantity",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Price",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Owner",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]).height(2),
    )
    .widths(&[
        Constraint::Percentage(55),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ]);
    ////////////////////////////////////////////////

    f.render_widget(items_table, chunks[0]);
    f.render_widget(people_list, chunks[1]);
}