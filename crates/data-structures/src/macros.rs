#[macro_export]
macro_rules! step {
    ($text:literal) => {{
        $crate::step!(format!("{}", $text))
    }};
    ($text:literal, $( $arg:expr ),* ) => {{
        $crate::step!(format_args!($text, $($arg,)*))
    }};
    ($text:expr) => {{
        let (bg, fg) = $minilisp_util::color::couple(line!() as usize);
        let text = $text.to_string();
        eprintln!(
            "{}{}",
            minilisp_util::color::ansi(
                $crate::location!(),
                bg.into(),
                fg.into(),
            ),
            minilisp_util::color::ansi(
                if text.is_empty() { String::new() } else { format!(" {}", text) },
                bg.into(),
                fg.into(),
            )
        );
    }};
    () => {{
        $crate::step!("")
    }};
}
#[macro_export]
macro_rules! step_test {
    ($text:literal) => {{
        $crate::step_test!(format!("{}", $text))
    }};
    ($text:literal, $( $arg:expr ),* ) => {{
        $crate::step_test!(format_args!($text, $($arg,)*))
    }};
    ($text:expr) => {{
        let (bg, fg) = $minilisp_util::color::couple(line!() as usize);
        let text = $text.to_string();
        let full_text =
            format!("{}:{} {}", $crate::function_name!(), line!(), &text);

        eprintln!(
            "\n{}\n{} {}",
            minilisp_util::color::bg(" ".repeat(full_text.len()), bg as usize),
            minilisp_util::color::ansi(
                $crate::location!(),
                fg.into(),
                bg.into(),
            ),
            minilisp_util::color::ansi(
                if text.is_empty() { String::new() } else { format!("{}", text) },
                bg.into(),
                fg.into(),
            ),
        );
    }};
    () => {{
        $crate::step_test!("")
    }};
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = name
            .strip_suffix("::f")
            .unwrap()
            .replace(format!("{}::", module_path!()).as_str(), "");
        name
    }};
}

#[macro_export]
macro_rules! location {
    () => {{
        let location = format!(
            "{}{}{}:{}",
            minilisp_util::color::fg($crate::function_name!(), 178),
            minilisp_util::color::fg(" file ", 231),
            $crate::filename!(),
            minilisp_util::color::fg(line!().to_string(), 49)
        );
        location
    }};
    (begin) => {
        $crate::tag!(minilisp_util::color::fg(format!("in function {}", $crate::location!()), 231))
    };
    (end) => {
        $crate::tag!(
            close,
            minilisp_util::color::fg(format!("from function {}", $crate::location!()), 231)
        )
    };
}
#[macro_export]
macro_rules! filename {
    () => {
        $crate::filename!(237, 49)
    };
    ($folder_color:literal, $file_color:literal) => {{
        let mut parts = file!()
            .split(std::path::MAIN_SEPARATOR_STR)
            .map(String::from)
            .collect::<Vec<String>>();
        let (folder, filename) = if parts.len() > 1 {
            let last = minilisp_util::color::fg(parts.remove(parts.len() - 1), $file_color);
            let mut parts = parts
                .iter()
                .map(|part| minilisp_util::color::fg(part, $folder_color))
                .collect::<Vec<String>>();
            (parts, last)
        } else {
            (
                Vec::<String>::new(),
                minilisp_util::color::fg(parts[0].to_string(), $file_color),
            )
        };
        if folder.len() > 1 {
            format!(
                "{}{}{}",
                filename,
                minilisp_util::color::fg(" in ", 7),
                folder.join(std::path::MAIN_SEPARATOR_STR)
            )
        } else {
            filename
        }
    }};
}

#[macro_export]
macro_rules! warn {
    ($text:literal) => {{
        $crate::warn!(format!("{}", $text))
    }};
    ($text:literal, $( $arg:expr ),* ) => {{
        $crate::warn!(format_args!($text, $($arg,)*))
    }};
    ($text:expr) => {{
        let bg = 231usize;
        let fg = 16usize;
        let text = $text.to_string();
        eprintln!(
            "{} {}",
            minilisp_util::color::ansi(
                $crate::location!(),
                fg.into(),
                bg.into(),
            ),
            minilisp_util::color::ansi(
                if text.is_empty() { String::new() } else { format!("{}", text) },
                bg.into(),
                fg.into(),
            )
        );
    }};
    () => {{
        $crate::warn!("")
    }};
}

#[macro_export]
macro_rules! warn_inv {
    ($text:literal) => {{
        $crate::warn_inv!(format!("{}", $text))
    }};
    ($text:literal, $( $arg:expr ),* ) => {{
        $crate::warn_inv!(format_args!($text, $($arg,)*))
    }};
    ($text:expr) => {{
        let bg = 231usize;
        let fg = 16usize;
        let text = $text.to_string();
        eprintln!(
            "{} {}",
            minilisp_util::color::ansi(
                $crate::location!(),
                bg.into(),
                fg.into(),
            ),
            minilisp_util::color::ansi(
                if text.is_empty() { String::new() } else { format!("{}", text) },
                fg.into(),
                bg.into(),
            )
        );
    }};
    () => {{
        $crate::warn_inv!("")
    }};
}

#[macro_export]
macro_rules! tag {
    ($arg:expr) => {{
        $crate::tag!($arg, 7)
    }};
    (close, $arg:expr) => {{
        $crate::tag!(close, $arg, 7)
    }};
    ($arg:expr, $color:literal) => {{
        format!("{}{}{}", minilisp_util::color::fg("<", $color), $arg, minilisp_util::color::fg(">", $color),)
    }};
    (close, $arg:expr, $color:literal) => {{
        format!("{}{}{}", minilisp_util::color::fg("</", $color), $arg, minilisp_util::color::fg(">", $color),)
    }};
}
#[macro_export]
macro_rules! dbg {
    () => {{
        eprintln!("");
    }};
    ($( $arg:expr ),* ) => {{
        let obj = format!("{}", [$(
            format!("{}", $crate::indent_objdump!($arg)),
        )*].iter().map(minilisp_util::color::reset).collect::<Vec<String>>().join("\n"));
        eprintln!("\n\r{}", minilisp_util::color::reset([$crate::location!(begin), obj, $crate::location!(end)].join("\n")));
    }};
}
#[macro_export]
macro_rules! indent_objdump {
    ($indentation:literal, $obj:expr) => {{
        format!("{:#?}", $obj)
            .lines()
            .map(|line| format!("{}{}", " ".repeat($indentation), line))
            .collect::<Vec<String>>()
            .join("\n")
    }};
    ($obj:expr) => {{
        $crate::indent_objdump!(4, $obj)
    }};
}
