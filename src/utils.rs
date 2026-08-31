use std::fmt;

use tokio::signal::unix::{SignalKind, signal};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub struct ShortUuid(pub Uuid);

impl fmt::Display for ShortUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.0.as_bytes();

        write!(
            f,
            "{:02x}{:02x}..{:02x}{:02x}",
            bytes[0], bytes[1], bytes[14], bytes[15],
        )
    }
}

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

#[cfg(test)]
mod tests {
    use super::ShortUuid;

    #[test]
    fn short_uuid_displays_the_first_and_last_four_hex_digits() {
        let id = uuid::Uuid::parse_str("12345678-1234-5678-90ab-cdef12345678").unwrap();

        assert_eq!(ShortUuid(id).to_string(), "1234..5678");
    }
}
