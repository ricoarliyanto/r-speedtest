use crate::common::error::{Error, Result};
use crate::common::result::ApiResponse;
use crate::common::validatedjson::VJson;
use axum::response::IntoResponse;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use tracing::{error, info};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug, Clone)]
pub struct AlertConfig {
    pub webhook_url: Option<String>,
    pub telegram_bot_token: Option<String>,
    pub telegram_chat_id: Option<String>,
    pub discord_webhook_url: Option<String>,
    pub email_recipients: Option<Vec<String>>,
    pub min_throughput_mbps_threshold: Option<f64>,
    pub max_packet_loss_percent_threshold: Option<f64>,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct SendAlertReq {
    pub title: String,
    pub message: String,
    pub severity: String, // "info", "warning", "critical"
    pub channel: Option<String>, // "all", "webhook", "telegram", "discord"
}

pub struct AlertDispatcher;

impl AlertDispatcher {
    pub async fn dispatch_alert(
        config: &AlertConfig,
        title: &str,
        message: &str,
        severity: &str,
    ) -> Result<()> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| Error::Message(format!("Failed to build HTTP client: {:?}", e)))?;

        // 1. Generic Webhook Dispatch
        if let Some(webhook) = &config.webhook_url {
            let payload = json!({
                "title": title,
                "message": message,
                "severity": severity,
                "timestamp": chrono::Utc::now().to_rfc3339()
            });
            let _ = client.post(webhook).json(&payload).send().await;
        }

        // 2. Telegram Bot Dispatch
        if let (Some(token), Some(chat_id)) = (&config.telegram_bot_token, &config.telegram_chat_id) {
            let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
            let text = format!("🚨 *{}*\nSeverity: `{}`\n\n{}", title, severity, message);
            let payload = json!({
                "chat_id": chat_id,
                "text": text,
                "parse_mode": "Markdown"
            });
            let _ = client.post(&url).json(&payload).send().await;
        }

        // 3. Discord Webhook Dispatch
        if let Some(discord_url) = &config.discord_webhook_url {
            let color = match severity {
                "critical" => 0xFF0000,
                "warning" => 0xFFA500,
                _ => 0x00FF00,
            };
            let payload = json!({
                "embeds": [{
                    "title": title,
                    "description": message,
                    "color": color,
                    "footer": { "text": "QiLuo NOC Telemetry Alert" }
                }]
            });
            let _ = client.post(discord_url).json(&payload).send().await;
        }

        info!("Dispatched NOC alert: {} [{}]", title, severity);
        Ok(())
    }
}

pub async fn test_alert(VJson(req): VJson<SendAlertReq>) -> impl IntoResponse {
    let mock_config = AlertConfig {
        webhook_url: None,
        telegram_bot_token: None,
        telegram_chat_id: None,
        discord_webhook_url: None,
        email_recipients: None,
        min_throughput_mbps_threshold: Some(50.0),
        max_packet_loss_percent_threshold: Some(5.0),
    };

    let _ = AlertDispatcher::dispatch_alert(
        &mock_config,
        &req.title,
        &req.message,
        &req.severity,
    )
    .await;

    ApiResponse::ok("Alert dispatched successfully")
}
