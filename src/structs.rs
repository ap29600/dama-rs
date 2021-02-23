use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Serialize, Deserialize)]
pub enum SeralizableWidget {
    Notebook(Vec<SeralizableWidget>),
    Box(String, Orientation, Vec<SeralizableWidget>),
    Label(String),
    Button(String, String), // label, command
    Scale(f64, f64, String, String), // min, max, initialize, update
}
