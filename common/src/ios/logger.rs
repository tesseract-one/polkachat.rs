use stderrlog::LogLevelNum;

pub (super) fn init() -> crate::Result<()> {
    let level = if cfg!(debug_assertions) {
        LogLevelNum::Debug
    } else {
        LogLevelNum::Error
    };

    stderrlog::new()
        .verbosity(level)
        .module("PolkaChat")
        .show_module_names(true)
        .init()?;

    Ok(())
}