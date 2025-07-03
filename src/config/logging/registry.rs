use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub async fn init_logging() {
    let file_appender = rolling::daily("logs", "api.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    Box::leak(Box::new(guard));

    tracing_subscriber::registry()
        .with(fmt::layer()
            .with_writer(std::io::stdout)
            .with_target(false)
            .with_ansi(false)
        )
        .with(fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false)
        )
        .with(EnvFilter::new("info,sqlx=off"))
        .init()
}
