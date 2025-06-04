pub fn fg<T: std::fmt::Display>(text: T, fg: usize) -> String {
    format!("\x1b[1;38;5;{}m{}", wrap(fg), text)
}
pub fn bg<T: std::fmt::Display>(text: T, bg: usize) -> String {
    format!("\x1b[1;48;5;{}m{}", wrap(bg), text)
}
pub fn reset<T: std::fmt::Display>(text: T) -> String {
    format!("{}\x1b[0m", text)
}
pub fn bgfg<T: std::fmt::Display>(text: T, fore: usize, back: usize) -> String {
    bg(fg(text, wrap(fore) as usize), wrap(back) as usize)
}
pub fn ansi<T: std::fmt::Display>(text: T, fore: usize, back: usize) -> String {
    reset(bgfg(text, fore as usize, back as usize))
}
pub fn fore<T: std::fmt::Display>(text: T, fore: usize) -> String {
    let (fore, back) = couple(fore);
    ansi(text, fore as usize, back as usize)
}
pub fn back<T: std::fmt::Display>(text: T, back: usize) -> String {
    let (back, fore) = couple(back);
    ansi(text, fore as usize, back as usize)
}

pub fn couple(color: usize) -> (u8, u8) {
    let fore = wrap(color);
    let back = invert_bw(fore);
    (fore, back)
}

pub fn invert_bw(color: u8) -> u8 {
    match color {
        0 | 8 | 16..24 | 232..237 => 255,
        _ => 16,
    }
}

pub fn wrap(color: usize) -> u8 {
    (if color > 0 { color % 255 } else { color }) as u8
}
