use serde_derive::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Copy, Clone, Eq, PartialEq)]
pub enum OrientationSerial {
    Vertical,
    Horizontal
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
pub struct Box {
    #[serde( default )]
    pub title: String,
    #[serde( default )]
    pub orientation: OrientationSerial,
    #[serde ( default )]
    pub children: Vec<SerializableWidget>
}


#[derive(Deserialize, Serialize, Clone)]
pub struct Image {
    pub path: String,
}


#[derive(Deserialize, Serialize, Clone)]
pub struct Button {
    #[serde( default = "default_button_label")]
    pub text: String,
    pub on_click: String,
}
fn default_button_label() -> String {
    "Button".to_string()
}


#[derive(Deserialize, Serialize, Clone)]
pub struct CheckBox {
    #[serde( default )]
    pub text: String,
    pub initialize: String,
    pub on_click: String,    
}


#[derive (Deserialize, Serialize, Clone)]
pub struct Range {
    pub low: f64,
    pub high: f64,
}

impl Default for Range {
    fn default() -> Range { Range {low: 0., high: 100.} }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Scale {
    #[serde( default )]
    pub range: Range,
    pub initialize: String,
    pub on_update: String
}


//TODO: implement sane defaults for the select command
#[derive(Deserialize, Serialize, Clone)]
pub struct ComboBox {
    pub initialize: String,
    pub select: String,
    pub on_update: String
}

#[derive(Deserialize, Serialize, Clone)]
pub enum SerializableWidget {
    Notebook(Vec<SerializableWidget>), // children
    Box(Box), // children
    Label(String), // text
    Image(Image), // path
    Button(Button), // label, command
    CheckBox(CheckBox), // label, initialize, update
    Scale(Scale), // min, max, initialize, update
    ComboBox(ComboBox),  // get list, get active, update
}


#[test]
fn deserialization() {
    let e = crate::ui_builder::generate_fallback_layout("hi".to_string());
    assert_eq!("", serde_yaml::to_string(&e).unwrap());
}
