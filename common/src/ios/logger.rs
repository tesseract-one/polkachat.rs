pub (super) fn init() -> crate::Result<()> {
    use stderrlog::LogLevelNum;

    let level = if cfg!(debug_assertions) {
        LogLevelNum::Debug
        //LogLevelNum::Trace
    } else {
        LogLevelNum::Error
    };

    Ok(stderrlog::new().verbosity(level).show_module_names(true).init()?)
}