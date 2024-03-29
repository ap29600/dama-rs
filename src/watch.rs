use std::sync::{Arc, Condvar, Mutex};

// the Watch struct is used for passing data to a spawned thread,
// allowing parallelization of processes that might take a long time
// to execute.
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
