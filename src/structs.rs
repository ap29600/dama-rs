use serde_derive::Deserialize;
use gtk::Orientation;

#[derive(Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde (remote = "Orientation")]
pub enum OrientationSerial {
    Vertical,
    Horizontal,
}

#[derive(Deserialize, Clone)]
pub enum SerializableWidget {
    Notebook(Vec<SerializableWidget>), // children
    Box(String,  // title, only used if parent is a Notebook
        // trick to derive Deserialize on gtk::Orientation
        #[serde (with = "OrientationSerial")] 
        Orientation,
        Vec<SerializableWidget>), // children
    Label(String), // text
    Image(String), // path
    Button(String, String), // label, command
    Scale(f64, f64, String, String), // min, max, initialize, update
}
