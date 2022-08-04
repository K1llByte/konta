pub struct AppState {
    pub focused: FocusedWindow,
    pub data: Data,
}

pub enum FocusedWindow {
    Items(usize),
    People(usize),
    OwnerSelector(usize,usize),
    AddPerson(String),
}

pub struct Data {
    pub items: Vec<Item>,
    pub people: Vec<String>,
}

#[derive(Clone)]
pub struct Item {
    pub description: String,
    pub quantity: u32,
    pub price: f32,
    pub owner: Option<usize>,
}

impl Default for AppState {
    fn default() -> Self {
        let item1 = Item{
            description: "Iogurte Grego Natural Açucarado".into(),
            quantity: 2,
            price: 2.48,
            owner: None,
        };
        let item2 = Item{
            description: "Iogurte Grego Natural Açucarado".into(),
            quantity: 1,
            price: 1.24,
            owner: None,
        };
        AppState{
            focused: FocusedWindow::Items(0),
            data: Data {
                items: vec![
                    item1.clone(),
                    item2.clone(),
                    item1.clone(),
                    item2.clone(),
                    item1.clone(),
                    item2.clone(),
                    item1.clone(),
                    item2.clone(),
                    item1.clone(),
                    item2.clone(),
                    item1.clone(),
                    item2.clone(),
                    item1.clone(),
                    item2.clone(),
                    item1.clone(),
                    item2.clone(),
                    item1.clone(),
                    item2.clone(),
                    item1.clone(),
                    item2.clone(),
                ],
                people: vec![
                    // "jojo".into(),
                    // "bu".into(),
                ]
            },
        }
    }
}

impl Data {
    pub fn set_item_owner(&mut self, item_idx: usize, opt_person_idx: Option<usize>) {
        self.items[item_idx].owner = opt_person_idx;
    }

    pub fn compute_total(&self) -> Vec<f32> {
        let mut totals = Vec::with_capacity(self.people.len());
        for _ in 0..self.people.len() {
            totals.push(0f32);
        }
        for (i, item) in self.items.iter().enumerate() {
            if let Some(owner_idx) = item.owner {
                totals[owner_idx] += item.price
            }
        }
        totals
    }
}

pub fn owner_to_string(owner: Option<usize>, app: &AppState) -> String {
    match owner {
        Some(idx) => app.data.people[idx].clone(),
        None => "".into(),
    }
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
    }
    else {
        Color::Reset
    }
    
}