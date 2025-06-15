#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        use minilisp_util::color;
        eprintln!(
            "\n{}",
            [
                color::ansi("WARNING:", 220, 16),
                color::ansi($message, 16, 220),
            ]
            .join(" ")
        );
    };
}
#[macro_export]
macro_rules! info {
    ($message:expr) => {
        use minilisp_util::color;
        eprintln!(
            "\n{}",
            [
                color::ansi("INFO:", 74, 16),
                color::ansi($message, 16, 74),
            ]
            .join(" ")
        );
    };
}
