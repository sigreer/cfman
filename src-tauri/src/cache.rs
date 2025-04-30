use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use super::models::CloudflareZone;

lazy_static::lazy_static! {
    static ref ZONE_CACHE: Mutex<HashMap<String, (Vec<CloudflareZone>, u64)>> = Mutex::new(HashMap::new());
}

const CACHE_DURATION: u64 = 300; // 5 minutes in seconds

pub fn get_cached_zones(account_id: &str) -> Option<Vec<CloudflareZone>> {
    let cache = ZONE_CACHE.lock().unwrap();
    if let Some((zones, timestamp)) = cache.get(account_id) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        if current_time - timestamp < CACHE_DURATION {
            return Some(zones.clone());
        }
    }
    None
}

pub fn set_cached_zones(account_id: &str, zones: Vec<CloudflareZone>) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
        
    let mut cache = ZONE_CACHE.lock().unwrap();
    cache.insert(account_id.to_string(), (zones, timestamp));
} 