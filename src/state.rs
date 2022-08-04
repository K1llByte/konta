use std::io;
use std::path::Path;

pub struct AppState {
    pub focused: FocusedWindow,
    pub data: Data,
}

pub enum FocusedWindow {
    Items(usize),
    People(usize),
    OwnerSelector(usize, usize, Vec<usize>),
    RestOwnerSelector(usize),
    AddPerson(String),
}

pub struct Data {
    pub items: Vec<Item>,
    pub people: Vec<String>,
}

pub struct Owner {
    pub person: usize,
    pub percentage: f32,
}

pub struct Item {
    pub description: String,
    pub quantity: u32,
    pub price: f32,
    pub owners: Vec<Owner>,
}

impl Default for AppState {
    fn default() -> Self {
        let item1 = Item {
            description: "Iogurte Grego Natural Açucarado".into(),
            quantity: 2,
            price: 2.48,
            owners: Vec::new(),
        };
        let item2 = Item {
            description: "Iogurte Grego Natural Açucarado".into(),
            quantity: 1,
            price: 1.24,
            owners: Vec::new(),
        };
        AppState {
            focused: FocusedWindow::Items(0),
            data: Data {
                items: vec![item1, item2],
                people: vec!["jojo".into()],
            },
        }
    }
}

impl AppState {
    pub fn with_data(data: Data) -> Self {
        Self {
            focused: FocusedWindow::Items(0),
            data,
        }
    }
}

impl Data {
    pub fn set_item_owner(&mut self, item_idx: usize, person_idx: usize) {
        let owner = Owner {
            person: person_idx,
            percentage: 1.0,
        };

        let size = self.items[item_idx].owners.len();
        // Special case to avoid reallocation
        if size == 1 {
            self.items[item_idx].owners[0] = owner;
        } else {
            self.items[item_idx].owners = vec![owner];
        }
    }

    pub fn set_item_owners(&mut self, item_idx: usize, owners: Vec<Owner>) {
        self.items[item_idx].owners = owners;
    }

    pub fn set_rest_items_owner(&mut self, person_idx: usize) {
        for item in &mut self.items {
            if item.owners.is_empty() {
                item.owners.push(Owner {
                    person: person_idx,
                    percentage: 1.0,
                })
            }
        }
    }

    pub fn compute_total(&self) -> Vec<f32> {
        let mut totals = Vec::with_capacity(self.people.len());
        for _ in 0..self.people.len() {
            totals.push(0f32);
        }
        for (i, item) in self.items.iter().enumerate() {
            for owner in &item.owners {
                totals[owner.person] += owner.percentage * item.price;
            }
        }
        totals
    }

    pub fn load<P: AsRef<Path>>(filename: P) -> io::Result<Self> {
        let mut items = Vec::<Item>::with_capacity(20);

        use regex::Regex;
        use std::fs::File;
        use std::io::BufRead;
        let file = File::open(filename)?;
        let mut parser_state: u8 = 0;
        // 0: Have to read description next
        // 1: Have to read quantity next
        // 2: Have to read discount next
        // 3: Have to read price next

        let mut current_item = Item {
            description: String::from(""),
            quantity: 0,
            price: 0.0,
            owners: Vec::new(),
        };
        for line in io::BufReader::new(file).lines() {
            let line = line.unwrap();
            match parser_state {
                0 => {
                    // 0. Item description (capture)
                    let re = Regex::new(r"[ ]{4}(.+)").unwrap();
                    if let Some(capture) = re.captures(&line) {
                        current_item.description = capture.get(1).unwrap().as_str().to_string();
                        parser_state = 1;
                    }
                }
                1 => {
                    // 1. Item quantity (capture)
                    let re = Regex::new(r"[ ]{4}(\d+).*").unwrap();
                    if let Some(capture) = re.captures(&line) {
                        current_item.quantity =
                            capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
                        parser_state = 2;
                    } else {
                        parser_state = 0;
                    }
                }
                2 => {
                    // 2. Item discount (match only)
                    let re = Regex::new(r"[ ]{4}-?\d+,\d+.*").unwrap();
                    if re.is_match(&line) {
                        parser_state = 3;
                    }
                }
                3 => {
                    // 3. Item price (capture)
                    let re = Regex::new(r"[ ]{4}(\d+),(\d+).*").unwrap();
                    if let Some(capture) = re.captures(&line) {
                        //
                        let a = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
                        let b = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
                        current_item.price = (a as f32) + ((b as f32) * 0.01f32);
                        if current_item.price != 0.0 {
                            items.push(current_item);
                            current_item = Item {
                                description: String::from(""),
                                quantity: 0,
                                price: 0.0,
                                owners: Vec::new(),
                            }
                        }
                    }
                    parser_state = 0;
                }
                _ => panic!("Unexpected data parser state"),
            }
        }
        Ok(Self {
            items,
            people: vec!["jojo".into()],
        })
    }
}

pub fn from_indices_to_owners(people: &Vec<usize>) -> Vec<Owner> {
    let mut owners = Vec::with_capacity(people.len());
    let percentage = 1f32 / (people.len() as f32);
    for person in people {
        owners.push(Owner {
            person: *person,
            percentage,
        });
    }
    owners
}

// [{"jojo",0.2}, {"jojo",0.2}, {"jojo",0.2}, {"jojo",0.2}, {"bu",0.2}]
// turns into
// [{"jojo",0.8}, {"bu",0.2}]
pub fn flatten_owners(owners: &mut Vec<Owner>) {
    owners.dedup_by(|a, b| {
        if a.person == b.person {
            b.percentage += a.percentage;
            true
        } else {
            false
        }
    });
}

pub fn owner_to_string(owner: &Owner, app: &AppState) -> String {
    app.data.people[owner.person].clone()
}

use tui::style::Color;
pub fn person_color(person: usize) -> Color {
    const VEC_COLORS: [Color; 14] = [
        Color::Blue,
        Color::Red,
        Color::LightCyan,
        Color::Green,
        Color::Yellow,
        Color::LightMagenta,
        Color::Magenta,
        Color::LightGreen,
        Color::Cyan,
        Color::LightYellow,
        Color::Gray,
        Color::LightBlue,
        Color::LightRed,
        Color::DarkGray,
    ];
    if person < 14 {
        VEC_COLORS[person]
    } else {
        Color::Reset
    }
}
