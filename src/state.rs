pub struct AppState {
    pub focused: FocusedWindow,
    pub data: Data,
}

pub enum FocusedWindow {
    Items(usize),
    People(usize),
    OwnerSelector(usize,usize),
}

pub struct Data {
    pub items: Vec<Item>,
    pub people: Vec<String>,
}

pub struct Item {
    pub description: String,
    pub quantity: u32,
    pub price: f32,
    pub owner: Option<usize>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState{
            focused: FocusedWindow::Items(0),
            data: Data {
                items: vec![
                    Item{
                        description: "Iogurte Grego Natural Açucarado".into(),
                        quantity: 2,
                        price: 2.48,
                        owner: None,
                    },
                    Item{
                        description: "Iogurte Grego Natural Açucarado".into(),
                        quantity: 2,
                        price: 2.48,
                        owner: None,
                    },
                    Item{
                        description: "Iogurte Grego Natural Açucarado".into(),
                        quantity: 2,
                        price: 2.48,
                        owner: Some(0),
                    },
                ],
                people: vec![
                    "jojo".into(),
                    "bu".into(),
                ]
            },
        }
    }
}

impl Data {
    pub fn set_item_owner(&mut self, item_idx: usize, opt_person_idx: Option<usize>) {
        self.items[item_idx].owner = opt_person_idx;
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