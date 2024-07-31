//! This module provides functionality for drawing various types of boxes and lines in a terminal environment.
//! It includes enums for different box styles, line styles, and shading options, as well as functions
//! to convert these enum variants to their corresponding Unicode characters.

use crate::*;
use std::{self, *};

/// Represents block characters for drawing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockChar {
    Full,
    UpperHalf,
    LowerHalf,
    LeftHalf,
    RightHalf,
    LightShade,
    MediumShade,
    DarkShade,
}

/// Represents different line styles for drawing lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineStyle {
    Solid,
    Dotted,
    Dashed,
    DoubleLine,
}

/// Represents shade styles for rectangles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadeStyle {
    Light,
    Medium,
    Dark,
    Solid,
}

/// Converts BlockChar enum variant to corresponding character.
///
/// # Arguments
///
/// * `ch` - The BlockChar variant to convert
///
/// # Returns
///
/// The Unicode character corresponding to the given BlockChar variant
pub fn block_char_to_char(ch: BlockChar) -> char {
    match ch {
        BlockChar::Full => '█',
        BlockChar::UpperHalf => '▀',
        BlockChar::LowerHalf => '▄',
        BlockChar::LeftHalf => '▌',
        BlockChar::RightHalf => '▐',
        BlockChar::LightShade => '░',
        BlockChar::MediumShade => '▒',
        BlockChar::DarkShade => '▓',
    }
}

/// Represents single line box characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleBox {
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    VerticalLeft,
    VerticalRight,
    HorizontalDown,
    HorizontalUp,
    VerticalHorizontal,
}

/// Represents double line box characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoubleBox {
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    VerticalLeft,
    VerticalRight,
    HorizontalDown,
    HorizontalUp,
    VerticalHorizontal,
}

/// Represents single line rounded box characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleRoundedBox {
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    VerticalLeft,
    VerticalRight,
    HorizontalDown,
    HorizontalUp,
    VerticalHorizontal,
}

/// Represents double line rounded box characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoubleRoundedBox {
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    VerticalLeft,
    VerticalRight,
    HorizontalDown,
    HorizontalUp,
    VerticalHorizontal,
}

/// Converts SingleBox enum variant to corresponding character.
///
/// # Arguments
///
/// * `ch` - The SingleBox variant to convert
///
/// # Returns
///
/// The Unicode character corresponding to the given SingleBox variant
pub fn single_box_to_char(ch: SingleBox) -> char {
    match ch {
        SingleBox::Horizontal => '─',
        SingleBox::Vertical => '│',
        SingleBox::TopLeft => '┌',
        SingleBox::TopRight => '┐',
        SingleBox::BottomLeft => '└',
        SingleBox::BottomRight => '┘',
        SingleBox::VerticalLeft => '├',
        SingleBox::VerticalRight => '┤',
        SingleBox::HorizontalDown => '┬',
        SingleBox::HorizontalUp => '┴',
        SingleBox::VerticalHorizontal => '┼',
    }
}

/// Converts DoubleBox enum variant to corresponding character.
///
/// # Arguments
///
/// * `ch` - The DoubleBox variant to convert
///
/// # Returns
///
/// The Unicode character corresponding to the given DoubleBox variant
pub fn double_box_to_char(ch: DoubleBox) -> char {
    match ch {
        DoubleBox::Horizontal => '═',
        DoubleBox::Vertical => '║',
        DoubleBox::TopLeft => '╔',
        DoubleBox::TopRight => '╗',
        DoubleBox::BottomLeft => '╚',
        DoubleBox::BottomRight => '╝',
        DoubleBox::VerticalLeft => '╠',
        DoubleBox::VerticalRight => '╣',
        DoubleBox::HorizontalDown => '╦',
        DoubleBox::HorizontalUp => '╩',
        DoubleBox::VerticalHorizontal => '╬',
    }
}

/// Converts SingleRoundedBox enum variant to corresponding character.
///
/// # Arguments
///
/// * `ch` - The SingleRoundedBox variant to convert
///
/// # Returns
///
/// The Unicode character corresponding to the given SingleRoundedBox variant
pub fn single_rounded_box_to_char(ch: SingleRoundedBox) -> char {
    match ch {
        SingleRoundedBox::Horizontal => '─',
        SingleRoundedBox::Vertical => '│',
        SingleRoundedBox::TopLeft => '╭',
        SingleRoundedBox::TopRight => '╮',
        SingleRoundedBox::BottomLeft => '╰',
        SingleRoundedBox::BottomRight => '╯',
        SingleRoundedBox::VerticalLeft => '├',
        SingleRoundedBox::VerticalRight => '┤',
        SingleRoundedBox::HorizontalDown => '┬',
        SingleRoundedBox::HorizontalUp => '┴',
        SingleRoundedBox::VerticalHorizontal => '┼',
    }
}

/// Converts DoubleRoundedBox enum variant to corresponding character.
///
/// # Arguments
///
/// * `ch` - The DoubleRoundedBox variant to convert
///
/// # Returns
///
/// The Unicode character corresponding to the given DoubleRoundedBox variant
pub fn double_rounded_box_to_char(ch: DoubleRoundedBox) -> char {
    match ch {
        DoubleRoundedBox::Horizontal => '═',
        DoubleRoundedBox::Vertical => '║',
        DoubleRoundedBox::TopLeft => '╒',
        DoubleRoundedBox::TopRight => '╕',
        DoubleRoundedBox::BottomLeft => '╘',
        DoubleRoundedBox::BottomRight => '╛',
        DoubleRoundedBox::VerticalLeft => '╞',
        DoubleRoundedBox::VerticalRight => '╡',
        DoubleRoundedBox::HorizontalDown => '╤',
        DoubleRoundedBox::HorizontalUp => '╧',
        DoubleRoundedBox::VerticalHorizontal => '╪',
    }
}

/// Represents different box drawing styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxStyle {
    Single,
    Double,
    SingleRounded,
    DoubleRounded,
    Dotted,
    Dashed,
}

/// Represents all possible box characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxChar {
    Single(SingleBox),
    Double(DoubleBox),
    SingleRounded(SingleRoundedBox),
    DoubleRounded(DoubleRoundedBox),
    Block(BlockChar),
}

/// Converts BoxChar enum variant to corresponding character.
///
/// # Arguments
///
/// * `ch` - The BoxChar variant to convert
///
/// # Returns
///
/// The Unicode character corresponding to the given BoxChar variant
pub fn box_char_to_char(ch: BoxChar) -> char {
    match ch {
        BoxChar::Single(single_ch) => single_box_to_char(single_ch),
        BoxChar::Double(double_ch) => double_box_to_char(double_ch),
        BoxChar::SingleRounded(single_rounded_ch) => single_rounded_box_to_char(single_rounded_ch),
        BoxChar::DoubleRounded(double_rounded_ch) => double_rounded_box_to_char(double_rounded_ch),
        BoxChar::Block(block_ch) => block_char_to_char(block_ch),
    }
}

/// Gets the appropriate character for a line segment based on the line style
///
/// # Arguments
///
/// * `style` - The LineStyle to use
/// * `is_vertical` - Whether the line is vertical (true) or horizontal (false)
///
/// # Returns
///
/// The Unicode character for the specified line style and orientation
fn get_line_char(style: LineStyle, is_vertical: bool) -> char {
    match style {
        LineStyle::Solid => if is_vertical { '│' } else { '─' },
        LineStyle::Dotted => if is_vertical { '┆' } else { '┄' },
        LineStyle::Dashed => if is_vertical { '┊' } else { '┈' },
        LineStyle::DoubleLine => if is_vertical { '║' } else { '═' },
    }
}

/// Draws a horizontal line with the specified style
///
/// # Arguments
///
/// * `x` - The starting x-coordinate
/// * `y` - The y-coordinate
/// * `width` - The width of the line
/// * `style` - The LineStyle to use
fn horizontal_line(x: u16, y: u16, width: u16, style: LineStyle) {
    move_cursor_to(x, y);
    let line_char = get_line_char(style, false);
    for _ in 0..width {
        put_char(line_char);
    }
}

/// Draws a vertical line with the specified style
///
/// # Arguments
///
/// * `x` - The x-coordinate
/// * `y` - The starting y-coordinate
/// * `height` - The height of the line
/// * `style` - The LineStyle to use
fn vertical_line(x: u16, y: u16, height: u16, style: LineStyle) {
    let line_char = get_line_char(style, true);
    for i in 0..height {
        move_cursor_to(x, y + i);
        put_char(line_char);
    }
}

/// Gets the appropriate corner characters for a box based on the box style
///
/// # Arguments
///
/// * `style` - The BoxStyle to use
///
/// # Returns
///
/// An array of four characters representing the top-left, top-right, bottom-left, and bottom-right corners
fn get_box_corners(style: BoxStyle) -> [char; 4] {
    match style {
        BoxStyle::Single => ['┌', '┐', '└', '┘'],
        BoxStyle::Double => ['╔', '╗', '╚', '╝'],
        BoxStyle::SingleRounded => ['╭', '╮', '╰', '╯'],
        BoxStyle::DoubleRounded => ['╒', '╕', '╘', '╛'],
        BoxStyle::Dotted => ['┌', '┐', '└', '┘'], // Using single box chars for corners
        BoxStyle::Dashed => ['┌', '┐', '└', '┘'], // Using single box chars for corners
    }
}

/// Draws a box with the specified style
///
/// # Arguments
///
/// * `x` - The x-coordinate of the top-left corner
/// * `y` - The y-coordinate of the top-left corner
/// * `width` - The width of the box
/// * `height` - The height of the box
/// * `style` - The BoxStyle to use
pub fn draw_box(x: u16, y: u16, width: u16, height: u16, style: BoxStyle) {
    let (viewport_width, viewport_height) = get_viewport();

    if x + width > viewport_width || y + height > viewport_height {
        handle_boundary_error("Box extends beyond viewport");
        return;
    }

    let corners = get_box_corners(style);
    let line_style = match style {
        BoxStyle::Single | BoxStyle::SingleRounded => LineStyle::Solid,
        BoxStyle::Double | BoxStyle::DoubleRounded => LineStyle::DoubleLine,
        BoxStyle::Dotted => LineStyle::Dotted,
        BoxStyle::Dashed => LineStyle::Dashed,
    };

    // Draw horizontal lines
    horizontal_line(x + 1, y, width - 2, line_style);
    horizontal_line(x + 1, y + height - 1, width - 2, line_style);

    // Draw vertical lines
    vertical_line(x, y + 1, height - 2, line_style);
    vertical_line(x + width - 1, y + 1, height - 2, line_style);

    // Draw corners
    move_cursor_to(x, y);
    put_char(corners[0]);
    move_cursor_to(x + width - 1, y);
    put_char(corners[1]);
    move_cursor_to(x, y + height - 1);
    put_char(corners[2]);
    move_cursor_to(x + width - 1, y + height - 1);
    put_char(corners[3]);
}

/// Draws a shaded rectangle
///
/// # Arguments
///
/// * `x` - The x-coordinate of the top-left corner
/// * `y` - The y-coordinate of the top-left corner
/// * `width` - The width of the rectangle
/// * `height` - The height of the rectangle
/// * `style` - The ShadeStyle to use
pub fn draw_shaded_rectangle(x: u16, y: u16, width: u16, height: u16, style: ShadeStyle) {
    let (viewport_width, viewport_height) = get_viewport();

    if x + width > viewport_width || y + height > viewport_height {
        handle_boundary_error("Rectangle extends beyond viewport");
        return;
    }

    let shade_char = match style {
        ShadeStyle::Light => block_char_to_char(BlockChar::LightShade),
        ShadeStyle::Medium => block_char_to_char(BlockChar::MediumShade),
        ShadeStyle::Dark => block_char_to_char(BlockChar::DarkShade),
        ShadeStyle::Solid => block_char_to_char(BlockChar::Full),
    };

    for dy in 0..height {
        move_cursor_to(x, y + dy);
        for _ in 0..width {
            put_char(shade_char);
        }
    }
}

/// Hides a box by overwriting it with spaces
///
/// # Arguments
///
/// * `x` - The x-coordinate of the top-left corner
/// * `y` - The y-coordinate of the top-left corner
/// * `width` - The width of the box
/// * `height` - The height of the box
pub fn hide_box(x: u16, y: u16, width: u16, height: u16) {
    let (viewport_width, viewport_height) = get_viewport();

    if x + width > viewport_width || y + height > viewport_height {
        handle_boundary_error("Box extends beyond viewport");
        return;
    }

    for dy in 0..height {
        move_cursor_to(x, y + dy);
        for _ in 0..width {
            put_char(' ');
        }
    }
}

/// Helper function to get the appropriate BoxChar based on style and type.
///
/// # Arguments
///
/// * `style` - The BoxStyle to use
/// * `char_type` - A string describing the type of box character (e.g., "Horizontal", "TopLeft")
///
/// # Returns
///
/// An Option containing the corresponding BoxChar if a match is found, or None if no match is found
pub fn get_box_char(style: BoxStyle, char_type: &str) -> Option<BoxChar> {
    match style {
        BoxStyle::Single => Some(BoxChar::Single(match char_type {
            "Horizontal" => SingleBox::Horizontal,
            "Vertical" => SingleBox::Vertical,
            "TopLeft" => SingleBox::TopLeft,
            "TopRight" => SingleBox::TopRight,
            "BottomLeft" => SingleBox::BottomLeft,
            "BottomRight" => SingleBox::BottomRight,
            "VerticalLeft" => SingleBox::VerticalLeft,
            "VerticalRight" => SingleBox::VerticalRight,
            "HorizontalDown" => SingleBox::HorizontalDown,
            "HorizontalUp" => SingleBox::HorizontalUp,
            "VerticalHorizontal" => SingleBox::VerticalHorizontal,
            _ => return None,
        })),
        BoxStyle::Double => Some(BoxChar::Double(match char_type {
            "Horizontal" => DoubleBox::Horizontal,
            "Vertical" => DoubleBox::Vertical,
            "TopLeft" => DoubleBox::TopLeft,
            "TopRight" => DoubleBox::TopRight,
            "BottomLeft" => DoubleBox::BottomLeft,
            "BottomRight" => DoubleBox::BottomRight,
            "VerticalLeft" => DoubleBox::VerticalLeft,
            "VerticalRight" => DoubleBox::VerticalRight,
            "HorizontalDown" => DoubleBox::HorizontalDown,
            "HorizontalUp" => DoubleBox::HorizontalUp,
            "VerticalHorizontal" => DoubleBox::VerticalHorizontal,
            _ => return None,
        })),
        BoxStyle::SingleRounded => Some(BoxChar::SingleRounded(match char_type {
            "Horizontal" => SingleRoundedBox::Horizontal,
            "Vertical" => SingleRoundedBox::Vertical,
            "TopLeft" => SingleRoundedBox::TopLeft,
            "TopRight" => SingleRoundedBox::TopRight,
            "BottomLeft" => SingleRoundedBox::BottomLeft,
            "BottomRight" => SingleRoundedBox::BottomRight,
            "VerticalLeft" => SingleRoundedBox::VerticalLeft,
            "VerticalRight" => SingleRoundedBox::VerticalRight,
            "HorizontalDown" => SingleRoundedBox::HorizontalDown,
            "HorizontalUp" => SingleRoundedBox::HorizontalUp,
            "VerticalHorizontal" => SingleRoundedBox::VerticalHorizontal,
            _ => return None,
        })),
        BoxStyle::DoubleRounded => Some(BoxChar::DoubleRounded(match char_type {
            "Horizontal" => DoubleRoundedBox::Horizontal,
            "Vertical" => DoubleRoundedBox::Vertical,
            "TopLeft" => DoubleRoundedBox::TopLeft,
            "TopRight" => DoubleRoundedBox::TopRight,
            "BottomLeft" => DoubleRoundedBox::BottomLeft,
            "BottomRight" => DoubleRoundedBox::BottomRight,
            "VerticalLeft" => DoubleRoundedBox::VerticalLeft,
            "VerticalRight" => DoubleRoundedBox::VerticalRight,
            "HorizontalDown" => DoubleRoundedBox::HorizontalDown,
            "HorizontalUp" => DoubleRoundedBox::HorizontalUp,
            "VerticalHorizontal" => DoubleRoundedBox::VerticalHorizontal,
            _ => return None,
        })),
        BoxStyle::Dotted | BoxStyle::Dashed => Some(BoxChar::Single(match char_type {
            "Horizontal" => SingleBox::Horizontal,
            "Vertical" => SingleBox::Vertical,
            "TopLeft" => SingleBox::TopLeft,
            "TopRight" => SingleBox::TopRight,
            "BottomLeft" => SingleBox::BottomLeft,
            "BottomRight" => SingleBox::BottomRight,
            "VerticalLeft" => SingleBox::VerticalLeft,
            "VerticalRight" => SingleBox::VerticalRight,
            "HorizontalDown" => SingleBox::HorizontalDown,
            "HorizontalUp" => SingleBox::HorizontalUp,
            "VerticalHorizontal" => SingleBox::VerticalHorizontal,
            _ => return None,
        })),
    }
}

/// Gets the corner character for the specified box style and corner type.
///
/// # Arguments
///
/// * `style` - The BoxStyle to use
/// * `corner` - A string describing the corner type ("TopLeft", "TopRight", "BottomLeft", or "BottomRight")
///
/// # Returns
///
/// An Option containing the corresponding corner character if a match is found, or None if no match is found
pub fn get_corner_char(style: BoxStyle, corner: &str) -> Option<char> {
    let corner_char = match style {
        BoxStyle::Single => match corner {
            "TopLeft" => single_box_to_char(SingleBox::TopLeft),
            "TopRight" => single_box_to_char(SingleBox::TopRight),
            "BottomLeft" => single_box_to_char(SingleBox::BottomLeft),
            "BottomRight" => single_box_to_char(SingleBox::BottomRight),
            _ => return None,
        },
        BoxStyle::Double => match corner {
            "TopLeft" => double_box_to_char(DoubleBox::TopLeft),
            "TopRight" => double_box_to_char(DoubleBox::TopRight),
            "BottomLeft" => double_box_to_char(DoubleBox::BottomLeft),
            "BottomRight" => double_box_to_char(DoubleBox::BottomRight),
            _ => return None,
        },
        BoxStyle::SingleRounded => match corner {
            "TopLeft" => single_rounded_box_to_char(SingleRoundedBox::TopLeft),
            "TopRight" => single_rounded_box_to_char(SingleRoundedBox::TopRight),
            "BottomLeft" => single_rounded_box_to_char(SingleRoundedBox::BottomLeft),
            "BottomRight" => single_rounded_box_to_char(SingleRoundedBox::BottomRight),
            _ => return None,
        },
        BoxStyle::DoubleRounded => match corner {
            "TopLeft" => double_rounded_box_to_char(DoubleRoundedBox::TopLeft),
            "TopRight" => double_rounded_box_to_char(DoubleRoundedBox::TopRight),
            "BottomLeft" => double_rounded_box_to_char(DoubleRoundedBox::BottomLeft),
            "BottomRight" => double_rounded_box_to_char(DoubleRoundedBox::BottomRight),
            _ => return None,
        },
        BoxStyle::Dotted | BoxStyle::Dashed => match corner {
            "TopLeft" => single_box_to_char(SingleBox::TopLeft),
            "TopRight" => single_box_to_char(SingleBox::TopRight),
            "BottomLeft" => single_box_to_char(SingleBox::BottomLeft),
            "BottomRight" => single_box_to_char(SingleBox::BottomRight),
            _ => return None,
        },
    };
    Some(corner_char)
}