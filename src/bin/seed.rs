use fake::{Fake, Faker, faker};
use iron_queue_rs::domain::jobs::{
    GenerateReportPayload, JobPayload, JobPriority, NewQueuedJob, ReportFormat, SendEmailPayload,
};
use iron_queue_rs::repository::jobs::JobRepository;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::error::Error;
use time::{Duration, OffsetDateTime};
use tracing::info;
use tracing_subscriber::EnvFilter;

use iron_queue_rs::env_config;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn")),
        )
        .with_target(false)
        .compact()
        .init();

    let config = env_config::EnvConfig::from_env()?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async_main(config))
}

async fn async_main(config: env_config::EnvConfig) -> Result<(), Box<dyn Error>> {
    info!("connecting to PostgreSQL");
    let pool = PgPoolOptions::new().connect(&config.database_url).await?;

    let repository = JobRepository::new(pool);

    for _ in 0..10 {
        let new_job = NewQueuedJob::new(
            faker::lorem::en::Words(1..4)
                .fake::<Vec<String>>()
                .join(" "),
            match Faker.fake::<u8>() % 2 {
                0 => JobPayload::SendEmail(get_fake_email_payload()),
                _ => JobPayload::GenerateReport(get_fake_report_payload()),
            },
            match Faker.fake::<u8>() % 3 {
                0 => JobPriority::Low,
                1 => JobPriority::Normal,
                _ => JobPriority::High,
            },
            u8::from(Faker.fake::<u8>() % 3 + 1),
        );

        let job = repository.insert_queued(new_job).await?;

        info!(job_id = %job.id, "new job added");
    }

    Ok(())
}

fn get_fake_email_payload() -> SendEmailPayload {
    SendEmailPayload {
        to: faker::internet::en::SafeEmail().fake(),
        subject: faker::finance::en::Bic().fake(),
        template_id: if Faker.fake::<u8>() < 180 {
            faker::currency::en::CurrencyName().fake()
        } else {
            "fail".to_string()
        },
        variables: HashMap::from([
            ("name".to_string(), faker::name::en::Name().fake()),
            (
                "company".to_string(),
                faker::company::en::CompanyName().fake(),
            ),
            ("city".to_string(), faker::address::en::CityName().fake()),
            (
                "phone".to_string(),
                faker::phone_number::en::PhoneNumber().fake(),
            ),
        ]),
    }
}

fn get_fake_report_payload() -> GenerateReportPayload {
    let now = OffsetDateTime::now_utc();
    let oldest_date = now - Duration::days((20..30).fake());
    let newest_date = now - Duration::days((10..20).fake());

    GenerateReportPayload {
        report_type: if Faker.fake::<u8>() < 180 {
            faker::currency::en::CurrencyName().fake()
        } else {
            "fail".to_string()
        },
        date_range_start: faker::time::en::DateTimeBetween(oldest_date, now).fake(),
        date_range_end: faker::time::en::DateTimeBetween(newest_date, now).fake(),
        format: match Faker.fake::<u8>() % 3 {
            0 => ReportFormat::Pdf,
            1 => ReportFormat::Csv,
            _ => ReportFormat::Excel,
        },
    }
}
