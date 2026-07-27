use tokio::signal::unix::{SignalKind, signal};
use tokio_util::sync::CancellationToken;

pub async fn shutdown_signal(token: CancellationToken) {
    let mut sigterm = match signal(SignalKind::terminate()) {
        Ok(signal) => signal,
        Err(error) => {
            tracing::error!(%error, "failed to register SIGTERM listener");
            token.cancel();
            return;
        }
    };

    let mut sigint = match signal(SignalKind::interrupt()) {
        Ok(signal) => signal,
        Err(error) => {
            tracing::error!(%error, "failed to register SIGINT listener");
            token.cancel();
            return;
        }
    };

    tokio::select! {
        _ = sigterm.recv() => tracing::warn!(signal = "SIGTERM", "shutdown requested"),
        _ = sigint.recv() => tracing::warn!(signal = "SIGINT", "shutdown requested"),
        _ = token.cancelled() => tracing::warn!("programmatic shutdown requested"),
    }

    token.cancel();
}
