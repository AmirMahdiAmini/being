use std::time::{SystemTime, UNIX_EPOCH};

pub fn timestamp()->usize{
    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as usize
}
pub fn now()->String{
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
} 