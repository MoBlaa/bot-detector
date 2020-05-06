use std::time::{Instant};
use std::sync::{Arc, RwLock};
use std::ops::Sub;
use chrono::{DateTime, NaiveDateTime, Utc, Duration};

pub type Entry = (NaiveDateTime, String);

type Entries = Arc<RwLock<Vec<Entry>>>;

/// Stores information about messages, senders and timestamp
pub struct Queue {
    timespan: Duration,
    max_size: usize,
    entries: Entries
}

impl Queue {
    pub fn new(max: usize, timespan: Duration) -> Queue {
        Queue {
            timespan,
            max_size: max,
            entries: Arc::new(RwLock::new(Vec::with_capacity(max)))
        }
    }

    /// Removes all entries from the queue older than [timespan]. Returns number of entries
    /// in the queue after removing old ones.
    fn remove_old(&self) -> usize {
        let now = Utc::now().naive_utc();
        let mut entries = self.entries.write()
            .expect("failed to get write lock for entries");
        entries.retain(|(timestamp, _)| now - *timestamp < self.timespan);
        entries.len()
    }

    /// Adds an entry returning if adding was successful.
    pub fn add(&self, entry: Entry) -> bool {
        let size = {
            let entries = self.entries.read()
                .expect("failed to get read lock on entries");
            entries.len()
        };
        let size = if size >= self.max_size {
            self.remove_old()
        } else {
            size
        };
        if size >= self.max_size {
            return false;
        }

        self.entries.write()
            .expect("failed to get write lock on entries")
            .push(entry);
        true
    }
}
