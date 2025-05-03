/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/swamp
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::{fmt, io};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    #[must_use]
    pub const fn code(&self) -> u8 {
        match self {
            Self::Black => 30,
            Self::Red => 31,
            Self::Green => 32,
            Self::Yellow => 33,
            Self::Blue => 34,
            Self::Magenta => 35,
            Self::Cyan => 36,
            Self::White => 37,
            Self::BrightBlack => 90,
            Self::BrightRed => 91,
            Self::BrightGreen => 92,
            Self::BrightYellow => 93,
            Self::BrightBlue => 94,
            Self::BrightMagenta => 95,
            Self::BrightCyan => 96,
            Self::BrightWhite => 97,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    fg_color: Option<Color>,
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl Style {
    #[must_use]
    pub const fn new() -> Self {
        Self { fg_color: None }
    }

    #[must_use]
    pub const fn fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }
}

#[must_use]
pub const fn style() -> Style {
    Style::new()
}

#[must_use]
pub const fn fg_black() -> Style {
    style().fg(Color::Black)
}
#[must_use]
pub const fn fg_red() -> Style {
    style().fg(Color::Red)
}
#[must_use]
pub const fn fg_green() -> Style {
    style().fg(Color::Green)
}
#[must_use]
pub const fn fg_yellow() -> Style {
    style().fg(Color::Yellow)
}
#[must_use]
pub const fn fg_blue() -> Style {
    style().fg(Color::Blue)
}
#[must_use]
pub const fn fg_magenta() -> Style {
    style().fg(Color::Magenta)
}
#[must_use]
pub const fn fg_cyan() -> Style {
    style().fg(Color::Cyan)
}
#[must_use]
pub const fn fg_white() -> Style {
    style().fg(Color::White)
}
#[must_use]
pub const fn fg_bright_black() -> Style {
    style().fg(Color::BrightBlack)
}
#[must_use]
pub const fn fg_bright_red() -> Style {
    style().fg(Color::BrightRed)
}
#[must_use]
pub const fn fg_bright_green() -> Style {
    style().fg(Color::BrightGreen)
}
#[must_use]
pub const fn fg_bright_yellow() -> Style {
    style().fg(Color::BrightYellow)
}
#[must_use]
pub const fn fg_bright_blue() -> Style {
    style().fg(Color::BrightBlue)
}
#[must_use]
pub const fn fg_bright_magenta() -> Style {
    style().fg(Color::BrightMagenta)
}
#[must_use]
pub const fn fg_bright_cyan() -> Style {
    style().fg(Color::BrightCyan)
}
#[must_use]
pub const fn fg_bright_white() -> Style {
    style().fg(Color::BrightWhite)
}

#[allow(clippy::missing_errors_doc)]
pub fn set_style(writer: &mut dyn Write, style: Style) -> io::Result<()> {
    write!(writer, "\x1b[0m")?;
    if let Some(color) = style.fg_color {
        write!(writer, "\x1b[{}m", color.code())?;
    }
    Ok(())
}
#[allow(clippy::missing_errors_doc)]
pub fn reset_style(writer: &mut dyn Write) -> io::Result<()> {
    write!(writer, "\x1b[0m")
}

#[allow(clippy::missing_errors_doc)]
pub fn print_styled<W: Write, T: Display>(
    writer: &mut W,
    style: Style,
    value: T,
) -> io::Result<()> {
    set_style(writer, style)?;
    write!(writer, "{value}")?;
    reset_style(writer)?;
    writer.flush()?;
    Ok(())
}

#[allow(clippy::missing_errors_doc)]
pub fn println_styled<W: Write, T: Display>(
    writer: &mut W,
    style: Style,
    value: T,
) -> io::Result<()> {
    set_style(writer, style)?;
    writeln!(writer, "{value}")?;
    reset_style(writer)?;
    writer.flush()?;
    Ok(())
}

pub struct StyledText<T: Display> {
    style: Style,
    value: T,
}

impl<T: Display> StyledText<T> {
    pub const fn new(style: Style, value: T) -> Self {
        Self { style, value }
    }
}

impl<T: Display> Display for StyledText<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(color) = self.style.fg_color {
            write!(f, "\x1b[{}m", color.code())?;
        }
        write!(f, "{}", self.value)?;
        write!(f, "\x1b[0m")?;
        Ok(())
    }
}

pub const fn color<T: Display>(color: Color, value: T) -> StyledText<T> {
    StyledText::new(style().fg(color), value)
}

pub const fn black<T: Display>(value: T) -> StyledText<T> {
    color(Color::Black, value)
}
pub const fn red<T: Display>(value: T) -> StyledText<T> {
    color(Color::Red, value)
}
pub const fn green<T: Display>(value: T) -> StyledText<T> {
    color(Color::Green, value)
}
pub const fn yellow<T: Display>(value: T) -> StyledText<T> {
    color(Color::Yellow, value)
}
pub const fn blue<T: Display>(value: T) -> StyledText<T> {
    color(Color::Blue, value)
}
pub const fn magenta<T: Display>(value: T) -> StyledText<T> {
    color(Color::Magenta, value)
}
pub const fn cyan<T: Display>(value: T) -> StyledText<T> {
    color(Color::Cyan, value)
}
pub const fn white<T: Display>(value: T) -> StyledText<T> {
    color(Color::White, value)
}
pub const fn bright_black<T: Display>(value: T) -> StyledText<T> {
    color(Color::BrightBlack, value)
}
pub const fn bright_red<T: Display>(value: T) -> StyledText<T> {
    color(Color::BrightRed, value)
}
pub const fn bright_green<T: Display>(value: T) -> StyledText<T> {
    color(Color::BrightGreen, value)
}
pub const fn bright_yellow<T: Display>(value: T) -> StyledText<T> {
    color(Color::BrightYellow, value)
}
pub const fn bright_blue<T: Display>(value: T) -> StyledText<T> {
    color(Color::BrightBlue, value)
}
pub const fn bright_magenta<T: Display>(value: T) -> StyledText<T> {
    color(Color::BrightMagenta, value)
}
pub const fn bright_cyan<T: Display>(value: T) -> StyledText<T> {
    color(Color::BrightCyan, value)
}
pub const fn bright_white<T: Display>(value: T) -> StyledText<T> {
    color(Color::BrightWhite, value)
}
