use mongodb::bson::{DateTime, Document};

pub trait DocumentWithTimestamps {
  fn with_timestamps(self) -> Document;
}

impl DocumentWithTimestamps for Document {
  fn with_timestamps(mut self) -> Document {
    let now = DateTime::now();
    self.insert("created_at", now);
    self.insert("updated_at", now);
    self
  }
}
