use tui::{
    Frame,
    backend::Backend,
    layout::{
        Constraint, Direction, Layout
    },
    widgets::{
        Block, Borders, Wrap, Paragraph,
        Cell, Row, Table, TableState, BorderType,
    },
    style::{
        Color, Modifier, Style
    },
    text::{
        Spans,
        Span,
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

    
    let side_chunks = if let FocusedWindow::AddPerson(_) = app.focused {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(85)].as_ref())
            .split(chunks[1])
    }
    else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(0), Constraint::Percentage(100)].as_ref())
            .split(chunks[1])
    };
    

    
    /////////////// Render people table /////////////

    let totals = app.data.compute_total();

    let mut people_rows = Vec::with_capacity(app.data.people.len());
    for (i,person) in app.data.people.iter().enumerate() {
        // Get row selected or not
        let row = match app.focused {
            // Selected
            FocusedWindow::People(idx) | FocusedWindow::OwnerSelector(_,idx,_) if i == idx => 
                Row::new(vec![
                    Cell::from(person.as_ref()),
                    Cell::from(totals[i].to_string()),
                ])
                .style(Style::default().bg(Color::White).fg(Color::Black)),
            // Normal
            _ => 
                Row::new(vec![
                    Cell::from(person.as_ref()),
                    Cell::from(totals[i].to_string()),
                ])
                .style(Style::default().bg(person_color(i))/* .fg(Color::Black) */),
        };
        people_rows.push(row);
    }
    
    // Create List and customize layout
    let people_list = Table::new(people_rows)
        .block(
            match app.focused {
                FocusedWindow::OwnerSelector(_,_,_) | FocusedWindow::People(_) =>
                    Block::default()
                        .borders(Borders::ALL)
                        .title("People")
                        .border_type(BorderType::Thick),
                _ => Block::default()
                    .borders(Borders::ALL)
                    .title("People"),
            }
        )
        .widths(&[
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ]);

    /////////////// Render items table /////////////
    let mut item_rows = Vec::with_capacity(app.data.items.len());

    for (i,item) in app.data.items.iter().enumerate() {
        // Get row selected or not
        let row = match app.focused {
            // Selected
            FocusedWindow::Items(idx) if i == idx =>  {
                Row::new(vec![
                        Cell::from(item.description.as_ref()),
                        Cell::from(item.quantity.to_string()),
                        Cell::from(item.price.to_string()),
                        Cell::from(
                            Spans::from({
                                let mut spans = Vec::with_capacity(item.owners.len());
                                for owner in &item.owners {
                                    spans.push(
                                        Span::styled(
                                            if owner.percentage == 1f32 {
                                                format!(" {} ", app.data.people[owner.person])
                                            }
                                            else {
                                                format!(" {} {:.2} ", app.data.people[owner.person], owner.percentage)
                                            },
                                            Style::default().bg(person_color(owner.person)).fg(Color::White)
                                        )
                                    );
                                    
                                }
                                spans
                            })
                        ),
                    ]
                )
                .style(Style::default().bg(Color::White).fg(Color::Black))
            },
            // Select respective owner
            FocusedWindow::OwnerSelector(idx,_,_) if i == idx => {
                Row::new(vec![
                        Cell::from(item.description.as_ref()),
                        Cell::from(item.quantity.to_string()),
                        Cell::from(item.price.to_string()),
                        Cell::from(
                            Spans::from({
                                let mut spans = Vec::with_capacity(item.owners.len());
                                for owner in &item.owners {
                                    spans.push(
                                        Span::styled(
                                            if owner.percentage == 1f32 {
                                                format!(" {} ", app.data.people[owner.person])
                                            }
                                            else {
                                                format!(" {} {:.2} ", app.data.people[owner.person], owner.percentage)
                                            },
                                            Style::default().bg(person_color(owner.person)).fg(Color::White)
                                        )
                                    );
                                    
                                }
                                spans
                            })
                        ),
                    ]
                )
                .style(Style::default().bg(Color::LightYellow).fg(Color::Black))
            },
            // Normal
            _ => {
                Row::new(vec![
                    Cell::from(item.description.as_ref()),
                    Cell::from(item.quantity.to_string()),
                    Cell::from(item.price.to_string()),
                    Cell::from(
                        Spans::from({
                            let mut spans = Vec::with_capacity(item.owners.len());
                            for owner in &item.owners {
                                spans.push(
                                    Span::styled(
                                        if owner.percentage == 1f32 {
                                            format!(" {} ", app.data.people[owner.person])
                                        }
                                        else {
                                            format!(" {} {:.2} ", app.data.people[owner.person], owner.percentage)
                                        },
                                        Style::default().bg(person_color(owner.person)).fg(Color::White)
                                    )
                                );
                                
                            }
                            spans
                        })
                    ),
                ])
            },
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
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Percentage(20),
    ]);

    /////////////// Render add person prompt ///////////////
    
    let new_person_name = if let FocusedWindow::AddPerson(name) = &app.focused {
        name.as_ref()
    }
    else {
        // This will never be rendered
        ""
    };

    let add_person_prompt = Paragraph::new(new_person_name.as_ref())
        .block(Block::default()
            .borders(Borders::ALL)
            .title("New Person")
            .border_type(BorderType::Thick)
        )
        // .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    
    ////////////////////////////////////////////////

    // Widgets state updates
    let mut items_state = TableState::default();
    let mut people_state = TableState::default();
    match app.focused {
        FocusedWindow::Items(idx) => {
            items_state.select(Some(idx));
        },
        FocusedWindow::OwnerSelector(item_idx, person_idx,_) => {
            items_state.select(Some(item_idx));
            people_state.select(Some(person_idx));
        },
        FocusedWindow::People(idx) => {
            people_state.select(Some(idx));
        },
        _ => {},
    };
    f.render_stateful_widget(items_table, chunks[0], &mut items_state);
    f.render_stateful_widget(people_list, side_chunks[1], &mut people_state);
    f.render_widget(add_person_prompt, side_chunks[0]);

    // f.render_widget(people_list, chunks[1]);
}