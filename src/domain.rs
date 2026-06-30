// ######################################################################
// ################################ Jobs ################################
// ######################################################################

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub kind: JobKind,
    pub priority: Priority,
    pub max_retries: u8,
}

#[derive(Serialize, Deserialize)]
pub enum JobKind {
    SyncData,
    SendEmail {
        to: String,
        body: String,
    },
    StartTimer {
        seconds: u32,
    },
    ProcessPayment {
        from: String,
        to: String,
        amount: f64,
    },
    SendNotification {
        id: u32,
        body: String,
    },
}

#[derive(Serialize, Deserialize)]
pub enum Priority {
    Low,
    Normal,
    High,
}
