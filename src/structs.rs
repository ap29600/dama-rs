use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Eq, PartialEq)]
pub enum OrientationSerial {
    Vertical,
    Horizontal,
}

impl Default for OrientationSerial {
    fn default() -> Self {
        OrientationSerial::Horizontal
    }
}

impl Into<gtk::Orientation> for OrientationSerial {
    fn into(self) -> gtk::Orientation {
        match self {
            OrientationSerial::Vertical => gtk::Orientation::Vertical,
            OrientationSerial::Horizontal => gtk::Orientation::Horizontal,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Label {
    pub text: String,
    pub css: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Box {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub orientation: OrientationSerial,
    #[serde(default)]
    pub children: Vec<SerializableWidget>,
    pub css: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Notebook {
    pub children: Vec<SerializableWidget>,
    pub css: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Image {
    pub path: String,
    pub css: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Button {
    #[serde(default = "default_button_label")]
    pub text: String,
    pub on_click: String,
    pub css: Option<String>,
}
fn default_button_label() -> String {
    "Button".to_string()
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CheckBox {
    #[serde(default)]
    pub text: String,
    pub initialize: String,
    pub on_click: String,
    pub css: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Range {
    pub low: f64,
    pub high: f64,
}

impl Default for Range {
    fn default() -> Range {
        Range {
            low: 0.,
            high: 100.,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Scale {
    #[serde(default)]
    pub range: Range,
    pub initialize: String,
    pub on_update: String,
    pub css: Option<String>,
}

//TODO: implement sane defaults for the select command
#[derive(Deserialize, Serialize, Clone)]
pub struct ComboBox {
    pub initialize: String,
    pub select: String,
    pub on_update: String,
    pub css: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum SerializableWidget {
    Notebook(Notebook),
    Box(Box),
    Label(Label),
    Image(Image),
    Button(Button),
    CheckBox(CheckBox),
    Scale(Scale),
    ComboBox(ComboBox),
}
