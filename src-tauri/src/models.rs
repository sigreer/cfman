use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudflareAccount {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub account_type: String,
    #[serde(default)]
    pub created_on: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloudflareZone {
    pub id: String,
    pub name: String,
    pub status: String,
    pub paused: bool,
    pub r#type: String,
    pub development_mode: i32,
    pub name_servers: Vec<String>,
    pub original_name_servers: Option<Vec<String>>,
    pub original_registrar: Option<String>,
    pub original_dnshost: Option<String>,
    pub modified_on: String,
    pub created_on: String,
    pub activated_on: String,
    pub meta: ZoneMeta,
    pub owner: ZoneOwner,
    pub account: ZoneAccount,
    pub tenant: Option<ZoneTenant>,
    pub tenant_unit: Option<ZoneTenantUnit>,
    pub permissions: Vec<String>,
    pub plan: ZonePlan,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZoneMeta {
    pub step: i32,
    pub custom_certificate_quota: i32,
    pub page_rule_quota: i32,
    pub phishing_detected: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZoneOwner {
    pub id: Option<String>,
    pub r#type: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZoneAccount {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZoneTenant {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZoneTenantUnit {
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZonePlan {
    pub id: String,
    pub name: String,
    pub price: i32,
    pub currency: String,
    pub frequency: String,
    pub is_subscribed: bool,
    pub can_subscribe: bool,
    pub legacy_id: String,
    pub legacy_discount: bool,
    pub externally_managed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudflareDnsRecord {
    pub id: String,
    pub zone_id: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub content: String,
    pub proxied: bool,
    pub ttl: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudflareError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultInfo {
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
    pub count: i32,
    pub total_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudflareResponse<T> {
    pub result: Vec<T>,
    pub result_info: Option<ResultInfo>,
    pub success: bool,
    pub errors: Vec<CloudflareError>,
    pub messages: Vec<String>,
} 