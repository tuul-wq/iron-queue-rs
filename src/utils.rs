use tokio::signal::unix::{SignalKind, signal};
use tokio_util::sync::CancellationToken;

pub async fn shutdown_signal(token: CancellationToken) {
    let mut sigterm = signal(SignalKind::terminate()).expect("failed to create sigterm signal");
    let mut sigint = signal(SignalKind::interrupt()).expect("failed to create sigint signal");

    tokio::select! {
        _ = sigterm.recv() => tracing::info!("sigterm received"),
        _ = sigint.recv() => tracing::info!("sigint received"),
        _ = token.cancelled() => {},
    }

    token.cancel();
}
