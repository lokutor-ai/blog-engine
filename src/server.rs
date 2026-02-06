use crate::engine::build_site;
use anyhow::Result;
use axum::Router;
use notify_debouncer_mini::{new_debouncer, notify::*};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use tower_http::services::ServeDir;

pub async fn serve<P: AsRef<Path>>(input: P, output: P, port: u16, include_drafts: bool) -> Result<()> {
    let input = input.as_ref().to_path_buf();
    let output = output.as_ref().to_path_buf();

    build_site(&input, &output, include_drafts)?;

    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_millis(500), tx)?;

    debouncer.watcher().watch(&input, RecursiveMode::Recursive)?;

    let input_cloned = input.clone();
    let output_cloned = output.clone();

    tokio::task::spawn_blocking(move || {
        while let Ok(res) = rx.recv() {
            match res {
                Ok(_) => {
                    if let Err(e) = build_site(&input_cloned, &output_cloned, include_drafts) {
                        tracing::error!("Rebuild failed: {}", e);
                    } else {
                        tracing::info!("Site rebuilt successfully.");
                    }
                }
                Err(e) => tracing::error!("Watch error: {:?}", e),
            }
        }
    });

    let app = Router::new().fallback_service(ServeDir::new(&output));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Serving blog at http://localhost:{}", port);
    axum::serve(listener, app).await?;

    Ok(())
}