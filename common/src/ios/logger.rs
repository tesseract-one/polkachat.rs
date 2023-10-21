pub (super) fn init() -> crate::Result<()> {
    use log::LogLevel;

    let level = if cfg!(debug_assertions) {
        LogLevel::Debug
        //LogLevelNum::Trace
    } else {
        LogLevel::Error
    };

    Ok(stderrlog::new().verbosity(level as usize).module("Polkachat").init()?)
}