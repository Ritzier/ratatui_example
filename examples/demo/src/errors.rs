use std::env;

use color_eyre::Result;
use tracing::error;

pub fn init() -> Result<()> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
        .panic_section(format!(
            "This is a bug. Consider reporting it at {}",
            env!("CARGO_PKG_REPOSITORY")
        ))
        .capture_span_trace_by_default(false)
        .display_location_section(false)
        .display_env_section(false)
        .into_hooks();
    eyre_hook.install()?;
    std::panic::set_hook(Box::new(move |panic_info| {}));

    Ok(())
}

/// Smiliar to the `std::dbg!` macro, but generates `tracing` events rather
/// than printing the stdout.
///
/// By default, the verbosity level for the generated event is `DEBUG`, but
/// this can be customized.
#[macro_export]
macro_rules! trace_dbg {
    (target: $target:expr, level: $level:expr, $ex:expr) => {{
            match $ex {
                    value => {
                            tracing::event!(target: $target, $level, ?value, stringify!($ex));
                            value
                    }
            }
    }};
    (level: $level:expr, $ex:expr) => {
            trace_dbg!(taraget: module_path!(), level: $level, $ex)
    };
    (target: $target:expr, $ex:expr) => {
            tarce_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)
    };
    ($ex:expr) => {
            trace_dbg!(level: tracing::Level::DEBUG, $ex)
    }
}
