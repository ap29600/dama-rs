use serde_derive::Deserialize;
use gtk::Orientation;
use std::sync::{Arc, Condvar, Mutex};

#[derive(Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde (remote = "Orientation")]
pub enum OrientationSerial {
    Vertical,
    Horizontal
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
    Checkbox(String, String, String), // label, initialize, update
    Scale(f64, f64, String, String), // min, max, initialize, update
    Combo(String, String, String),  // get list, get active, update
}


#[derive(Clone)]
pub struct Watch<T> {
    inner: Arc<WatchInner<T>>,
    last_seen_version: u64,
}

struct WatchInner<T> {
    value: Mutex<WatchContainer<T>>,
    on_update: Condvar,
}
struct WatchContainer<T> {
    value: T,
    version: u64,
}

impl<T: Clone> Watch<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(WatchInner {
                value: Mutex::new(WatchContainer {
                    value,
                    version: 1,
                }),
                on_update: Condvar::new(),
            }),
            last_seen_version: 0,
        }
    }
    
    /// Waits for the value to change, then returns a clone of the new value.
    pub fn wait(&mut self) -> T {
        let mut lock = self.inner.value.lock().unwrap();
        
        // Wait until the value is updated.
        while lock.version == self.last_seen_version {
            lock = self.inner.on_update.wait(lock).unwrap();
        }
        
        // Return the new value.
        self.last_seen_version = lock.version;
        lock.value.clone()
    }
    
    
    /// Updates the shared value and notifies all threads currently sleeping
    /// on a call to `wait`.
    pub fn set_value(&mut self, new_value: T) {
        let mut lock = self.inner.value.lock().unwrap();
        lock.value = new_value;
        lock.version += 1;
        drop(lock);
        
        self.inner.on_update.notify_all();
    }
}

