// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cloudflare;
mod models;
mod cache;

use cloudflare::{fetch_accounts as fetch_cloudflare_accounts, fetch_zones as fetch_cloudflare_zones, fetch_dns_records as fetch_cloudflare_dns_records};
use models::{CloudflareAccount, CloudflareZone, CloudflareDnsRecord};
use cache::{get_cached_zones, set_cached_zones};
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    email: String,
    api_key: String,
}

fn load_credentials() -> Result<Credentials, String> {
    let app_dir = tauri::api::path::app_dir(&tauri::Config::default()).ok_or("Could not get app directory")?;
    let credentials_path = app_dir.join("credentials.json");
    
    if !credentials_path.exists() {
        return Err("No credentials found".to_string());
    }
    
    let contents = fs::read_to_string(credentials_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&contents).map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_credentials() -> Result<bool, String> {
    load_credentials().map(|_| true).map_err(|_| false.to_string())
}

#[tauri::command]
async fn save_credentials(email: String, api_key: String) -> Result<(), String> {
    let app_dir = tauri::api::path::app_dir(&tauri::Config::default()).ok_or("Could not get app directory")?;
    let credentials_path = app_dir.join("credentials.json");
    
    let credentials = Credentials { email, api_key };
    let contents = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    
    fs::write(credentials_path, contents).map_err(|e| e.to_string())
}

#[tauri::command]
async fn fetch_accounts_command() -> Result<Vec<CloudflareAccount>, String> {
    let credentials = load_credentials()?;
    fetch_cloudflare_accounts(&credentials.email, &credentials.api_key)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_cached_zones_command(account_id: String) -> Result<Vec<CloudflareZone>, String> {
    match get_cached_zones(&account_id) {
        Some(zones) => Ok(zones),
        None => Ok(Vec::new()),
    }
}

#[tauri::command]
async fn fetch_zones_command(account_id: String) -> Result<Vec<CloudflareZone>, String> {
    let credentials = load_credentials()?;
    let zones = fetch_cloudflare_zones(&credentials.email, &credentials.api_key, &account_id)
        .await
        .map_err(|e| e.to_string())?;
    set_cached_zones(&account_id, zones.clone());
    Ok(zones)
}

#[tauri::command]
async fn fetch_dns_records_command(zone_id: String) -> Result<Vec<CloudflareDnsRecord>, String> {
    let credentials = load_credentials()?;
    fetch_cloudflare_dns_records(&credentials.email, &credentials.api_key, &zone_id)
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_credentials,
            save_credentials,
            fetch_accounts_command,
            fetch_zones_command,
            get_cached_zones_command,
            fetch_dns_records_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
