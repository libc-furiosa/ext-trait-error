use tracing::info_span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

fn main() {
    let span = info_span!("TEST");
    let _ = span.context();

    println!("Hello, world!");
}
