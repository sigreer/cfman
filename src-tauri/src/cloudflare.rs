use super::models::{CloudflareAccount, CloudflareZone, CloudflareDnsRecord, CloudflareError, CloudflareResponse};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use std::error::Error;
use std::fmt;

const API_BASE: &str = "https://api.cloudflare.com/client/v4";

pub async fn fetch_accounts(email: &str, api_token: &str) -> Result<Vec<CloudflareAccount>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("X-Auth-Key", HeaderValue::from_str(api_token)?);
    headers.insert("X-Auth-Email", HeaderValue::from_str(email)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    info!("Making request with email: {} and auth key length: {}", email, api_token.len());
    
    let response = client
        .get(&format!("{}/accounts", API_BASE))
        .headers(headers)
        .send()
        .await?;
        
    // Log the raw response text
    let response_text = response.text().await?;
    info!("Cloudflare API Response: {}", response_text);
    
    // Parse the response text
    let parsed_response: CloudflareResponse<CloudflareAccount> = serde_json::from_str(&response_text)
        .map_err(|e| {
            error!("Failed to parse response: {}", e);
            e
        })?;

    if !parsed_response.success {
        let error_msg = parsed_response.errors.first()
            .map(|e| format!("Error {}: {}", e.code, e.message))
            .unwrap_or_else(|| "Unknown error".to_string());
        error!("Cloudflare API error: {}", error_msg);
        anyhow::bail!(error_msg);
    }

    Ok(parsed_response.result)
}

pub async fn fetch_zones(email: &str, api_token: &str, account_id: &str) -> Result<Vec<CloudflareZone>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("X-Auth-Key", HeaderValue::from_str(api_token)?);
    headers.insert("X-Auth-Email", HeaderValue::from_str(email)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let mut all_zones = Vec::new();
    let mut page = 1;
    let per_page = 50;

    info!("Fetching zones for account {} with email {}", account_id, email);

    loop {
        let url = format!(
            "{}/zones?account.id={}&page={}&per_page={}",
            API_BASE, account_id, page, per_page
        );
        
        info!("Fetching zones from URL: {}", url);
        
        let response = client
            .get(&url)
            .headers(headers.clone())
            .send()
            .await?;
            
        let response_text = response.text().await?;
        info!("Zones API Response (page {}): {}", page, response_text);
        
        let parsed_response: CloudflareResponse<CloudflareZone> = serde_json::from_str(&response_text)?;

        if !parsed_response.success {
            let error_msg = parsed_response.errors.first()
                .map(|e| format!("Error {}: {}", e.code, e.message))
                .unwrap_or_else(|| "Unknown error".to_string());
            error!("Cloudflare API error: {}", error_msg);
            anyhow::bail!(error_msg);
        }

        info!("Page {}: Found {} zones", page, parsed_response.result.len());
        all_zones.extend(parsed_response.result);

        if let Some(result_info) = parsed_response.result_info {
            info!("Result info: page {}/{}, total: {}", page, result_info.total_pages, result_info.total_count);
            if page >= result_info.total_pages {
                break;
            }
        } else {
            info!("No result info available, stopping after first page");
            break;
        }

        page += 1;
    }

    info!("Total zones fetched: {}", all_zones.len());

    // Fetch analytics for each zone
    for zone in &mut all_zones {
        let analytics_url = format!("{}/zones/{}/analytics/dashboard", API_BASE, zone.id);
        let response = client
            .get(&analytics_url)
            .headers(headers.clone())
            .send()
            .await?;
            
        let response_text = response.text().await?;
        if let Ok(analytics) = serde_json::from_str::<serde_json::Value>(&response_text) {
            if let Some(visitors) = analytics["result"]["totals"]["visitors"].as_i64() {
                zone.visitors = Some(visitors);
            }
        }
    }

    info!("Fetched total of {} zones with analytics", all_zones.len());
    Ok(all_zones)
}

pub async fn fetch_dns_records(email: &str, api_token: &str, zone_id: &str) -> Result<Vec<CloudflareDnsRecord>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("X-Auth-Key", HeaderValue::from_str(api_token)?);
    headers.insert("X-Auth-Email", HeaderValue::from_str(email)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let response = client
        .get(&format!("{}/zones/{}/dns_records", API_BASE, zone_id))
        .headers(headers)
        .send()
        .await?;
        
    let response_text = response.text().await?;
    info!("DNS Records API Response: {}", response_text);
    
    let parsed_response: CloudflareResponse<CloudflareDnsRecord> = serde_json::from_str(&response_text)?;

    if !parsed_response.success {
        let error_msg = parsed_response.errors.first()
            .map(|e| format!("Error {}: {}", e.code, e.message))
            .unwrap_or_else(|| "Unknown error".to_string());
        anyhow::bail!(error_msg);
    }

    Ok(parsed_response.result)
} 