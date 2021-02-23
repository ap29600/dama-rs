use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SerializableWidget {
    Notebook(Vec<SerializableWidget>),
    Box(String, Orientation, Vec<SerializableWidget>),
    Label(String),
    Button(String, String), // label, command
    Scale(f64, f64, String, String), // min, max, initialize, update
}
