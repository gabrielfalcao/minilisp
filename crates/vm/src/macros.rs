#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        use minilisp_util::color;
        eprintln!(
            "{}",
            [color::ansi("WARNING:", 220, 16), color::ansi($message, 16, 220),].join("")
        );
    };
}
