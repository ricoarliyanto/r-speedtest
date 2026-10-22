use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bytes::Bytes;
use futures::StreamExt;
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use std::time::{Duration, Instant};
use chrono::Utc;
use crate::common::error::{Error, Result};

lazy_static::lazy_static! {
    static ref UPLOAD_CHUNK_1MB: Bytes = Bytes::from(vec![0x55; 1024 * 1024]);
}

/// HTTP Server Handler for TR-143 Upload Diagnostics:
/// Sinks incoming stream without disk writes to accurately measure upload bandwidth.
pub async fn tr143_upload_handler(body: Body) -> impl IntoResponse {
    let start_time = Instant::now();
    let bom_timestamp = Utc::now().timestamp_micros();
    let mut total_bytes_received: u64 = 0;

    let mut stream = body.into_data_stream();
    while let Some(chunk_res) = stream.next().await {
        if let Ok(chunk) = chunk_res {
            total_bytes_received += chunk.len() as u64;
        }
    }

    let duration_sec = start_time.elapsed().as_secs_f64();
    let eom_timestamp = Utc::now().timestamp_micros();

    let throughput_bps = if duration_sec > 0.0 {
        (total_bytes_received as f64 * 8.0) / duration_sec
    } else {
        0.0
    };
    let throughput_mbps = throughput_bps / 1_000_000.0;

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "bom_time_us": bom_timestamp,
            "eom_time_us": eom_timestamp,
            "total_bytes_received": total_bytes_received,
            "duration_sec": (duration_sec * 100.0).round() / 100.0,
            "throughput_mbps": (throughput_mbps * 100.0).round() / 100.0
        })),
    )
}

/// Client Diagnostic Runner for TR-143 Upload Test
pub async fn execute_tr143_upload_client(
    target_url: &str,
    total_size_mb: u64,
    connections: usize,
    timeout_sec: u64,
) -> Result<serde_json::Value> {
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_sec))
        .tcp_nodelay(true)
        .build()
        .map_err(|e| Error::Message(format!("Failed to build HTTP client: {:?}", e)))?;

    let client = Arc::new(client);
    let mut handles = Vec::new();
    let num_conn = connections.max(1).min(16);
    let size_per_conn_mb = (total_size_mb / num_conn as u64).max(1);

    let test_start = Instant::now();
    let bom_timestamp = Utc::now().timestamp_micros();

    for _ in 0..num_conn {
        let c = client.clone();
        let url = target_url.to_string();

        let handle = tokio::spawn(async move {
            let chunk = UPLOAD_CHUNK_1MB.clone();
            let mut payload = Vec::with_capacity((size_per_conn_mb * 1024 * 1024) as usize);
            for _ in 0..size_per_conn_mb {
                payload.extend_from_slice(&chunk);
            }

            let bytes_len = payload.len() as u64;
            let req_start = Instant::now();
            let resp = c.post(&url)
                .body(payload)
                .send()
                .await?;
            let _ = resp.text().await?;
            let rom_time_us = req_start.elapsed().as_micros() as u64;

            Ok::<(u64, u64), reqwest::Error>((bytes_len, rom_time_us))
        });
        handles.push(handle);
    }

    let mut total_bytes_sent: u64 = 0;
    let mut rom_times = Vec::new();

    for h in handles {
        if let Ok(Ok((bytes, rom_us))) = h.await {
            total_bytes_sent += bytes;
            rom_times.push(rom_us);
        }
    }

    let duration_sec = test_start.elapsed().as_secs_f64();
    let eom_timestamp = Utc::now().timestamp_micros();

    let avg_rom_ms = if !rom_times.is_empty() {
        (rom_times.iter().sum::<u64>() as f64 / rom_times.len() as f64) / 1000.0
    } else {
        0.0
    };

    let throughput_bps = if duration_sec > 0.0 {
        (total_bytes_sent as f64 * 8.0) / duration_sec
    } else {
        0.0
    };
    let throughput_mbps = throughput_bps / 1_000_000.0;

    Ok(json!({
        "diagnostic_type": "TR-143 Upload",
        "target_url": target_url,
        "connections": num_conn,
        "bom_time_us": bom_timestamp,
        "eom_time_us": eom_timestamp,
        "rom_time_ms": (avg_rom_ms * 100.0).round() / 100.0,
        "total_bytes_sent": total_bytes_sent,
        "duration_sec": (duration_sec * 100.0).round() / 100.0,
        "throughput_mbps": (throughput_mbps * 100.0).round() / 100.0,
        "is_success": total_bytes_sent > 0
    }))
}
