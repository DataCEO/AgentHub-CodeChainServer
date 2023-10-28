
#[macro_export]
macro_rules! clog {
    ($lvl:expr, $($arg:tt)+) => ({
        log!(target: "agent-hub", $lvl, $($arg)*);
    });
}
