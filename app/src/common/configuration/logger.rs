use tower_http::{
    trace::{DefaultMakeSpan, TraceLayer, DefaultOnFailure, DefaultOnEos},
};
use tracing::{subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::{self, MakeWriter},
    layer::SubscriberExt,
    EnvFilter, Registry,
};

pub fn setup_logger<Sink>(sink: Sink)
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let format_layer = fmt::layer()
        .pretty()
        .with_ansi(true)
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_timer(fmt::time::UtcTime::rfc_3339())
        .with_writer(sink);

    let subscriber = Registry::default().with(env_filter).with(format_layer);
    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");
}

pub fn get_trace_layer() -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_failure(DefaultOnFailure::new().level(Level::ERROR))
        .on_eos(DefaultOnEos::new().level(Level::DEBUG))
}
