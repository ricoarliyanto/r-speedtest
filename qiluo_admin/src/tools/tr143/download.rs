use axum::{
    body::Body,
    extract::Query,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use bytes::Bytes;
use futures::stream;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::{Duration, Instant};
use chrono::Utc;
use crate::common::error::{Error, Result};

lazy_static::lazy_static! {
    static ref ZERO_CHUNK_1MB: Bytes = Bytes::from(vec![0xAA; 1024 * 1024]);
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tr143DownloadQuery {
    pub size: Option<u64>,
}

/// HTTP Server Handler for TR-143 Download Diagnostics:
/// Streams chunked payload from zero-copy memory buffers to measure network line-rate throughput.
pub async fn tr143_download_handler(Query(query): Query<Tr143DownloadQuery>) -> impl IntoResponse {
    let total_size = query.size.unwrap_or(50 * 1024 * 1024); // default 50MB
    let chunk_size = 1024 * 1024; // 1MB
    let num_chunks = (total_size + chunk_size - 1) / chunk_size;

    let chunk = ZERO_CHUNK_1MB.clone();
    let stream = stream::repeat(chunk)
        .take(num_chunks as usize)
        .map(Ok::<Bytes, Infallible>);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/octet-stream"));
    headers.insert(header::CONTENT_LENGTH, HeaderValue::from_str(&total_size.to_string()).unwrap());
    headers.insert("X-TR143-Server", HeaderValue::from_static("QiLuo-TR143-Engine/2.0"));

    (StatusCode::OK, headers, body)
}

/// Client Diagnostic Runner for TR-143 Download Test
pub async fn execute_tr143_download_client(
    target_url: &str,
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

    let test_start = Instant::now();
    let bom_timestamp = Utc::now().timestamp_micros();

    for _ in 0..num_conn {
        let c = client.clone();
        let url = target_url.to_string();

        let handle = tokio::spawn(async move {
            let req_start = Instant::now();
            let mut resp = c.get(&url).send().await?;
            let rom_time_us = req_start.elapsed().as_micros() as u64;

            let mut total_bytes: u64 = 0;
            while let Some(chunk) = resp.chunk().await? {
                total_bytes += chunk.len() as u64;
            }

            Ok::<(u64, u64), reqwest::Error>((total_bytes, rom_time_us))
        });
        handles.push(handle);
    }

    let mut total_bytes_received: u64 = 0;
    let mut rom_times = Vec::new();

    for h in handles {
        if let Ok(Ok((bytes, rom_us))) = h.await {
            total_bytes_received += bytes;
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
        (total_bytes_received as f64 * 8.0) / duration_sec
    } else {
        0.0
    };
    let throughput_mbps = throughput_bps / 1_000_000.0;

    Ok(json!({
        "diagnostic_type": "TR-143 Download",
        "target_url": target_url,
        "connections": num_conn,
        "bom_time_us": bom_timestamp,
        "eom_time_us": eom_timestamp,
        "rom_time_ms": (avg_rom_ms * 100.0).round() / 100.0,
        "total_bytes_received": total_bytes_received,
        "duration_sec": (duration_sec * 100.0).round() / 100.0,
        "throughput_mbps": (throughput_mbps * 100.0).round() / 100.0,
        "is_success": total_bytes_received > 0
    }))
}
