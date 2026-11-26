//! Install the command's optional structured diagnostic stream.
//! spec: docs/repo/lint-rules.md#bindings
use std::fmt;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

/// UTC timestamps with millisecond precision.
struct UtcMillis;
impl FormatTime for UtcMillis {
    fn format_time(&self, writer: &mut Writer<'_>) -> fmt::Result {
        #[expect(
            clippy::disallowed_methods,
            reason = "The diagnostic sink timestamps external events."
        )]
        let timestamp = jiff::Timestamp::now();
        write!(writer, "{timestamp:.3}")
    }
}

/// Configure tracing only when the caller requested backend diagnostics.
pub(super) fn install(verbose: bool) -> Result<(), super::Failure> {
    if verbose {
        tracing_subscriber::fmt()
            .json()
            .with_writer(std::io::stderr)
            .with_timer(UtcMillis)
            .with_max_level(tracing::Level::DEBUG)
            .try_init()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
