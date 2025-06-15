#[macro_export]
macro_rules! admonition {
    ($title:literal, $message:expr, $color:literal) => {
        use minilisp_util::color;
        eprintln!(
            "\n{}",
            [
                color::ansi(format!("{}:", $title), $color, color::invert_bw($color).into()),
                color::ansi($message, color::invert_bw($color).into(), $color),
            ]
            .join(" ")
        );
    };
}

#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        $crate::warn!($message, 220)
    };
    ($message:expr, $color:literal) => {
        $crate::admonition!("WARNING", $message, $color)
    };
}


#[macro_export]
macro_rules! info {
    ($message:expr) => {
        $crate::info!($message, 74)
    };
    ($message:expr, $color:literal) => {
        $crate::admonition!("INFO", $message, $color)
    };
}
