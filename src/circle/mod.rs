#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircleSymbol {
    // Basic circles
    Circle,                    // ○ U+25CB
    FilledCircle,               // ● U+25CF
    // Geometric circles
    LargeCircle,               // ◯ U+25EF
    MediumFilledCircle,         // ⬤ U+2B24
    // Dotted and dashed circles
    DottedCircle,              // ◌ U+25CC
    CircleWithLeftHalfBlack,   // ◐ U+25D0
    CircleWithRightHalfBlack,  // ◑ U+25D1
    // Circles with inscribed shapes
    CircledDot,                // ◍ U+25CD
    CircleWithVerticalFill,    // ◓ U+25D3
    CircleWithHorizontalFill,  // ◒ U+25D2
    // Bullseye circles
    Bullseye,                  // ◎ U+25CE
    // Specialized circles
    SunSymbol,                 // ☉ U+2609
    FishEye,                   // ◉ U+25C9
    // Circles with additional elements
    CircleWithTwoDotsInside,   // ⚇ U+2687
    FilledCircleWithTwoDotsInside, // ⚉ U+2689
    // Emoji-style circles
    RedCircle,                 // 🔴 U+1F534
    BlueCircle,                // 🔵 U+1F535
    // Mathematical circles
    CircledPlus,               // ⊕ U+2295
    CircledMinus,              // ⊖ U+2296
    CircledTimes,              // ⊗ U+2297
}

/// Converts a CircleSymbol to its corresponding Unicode character.
pub fn circle_symbol_to_char(symbol: CircleSymbol) -> char {
    match symbol {
        CircleSymbol::Circle => '○',
        CircleSymbol::FilledCircle => '●',
        CircleSymbol::LargeCircle => '◯',
        CircleSymbol::MediumFilledCircle => '⬤',
        CircleSymbol::DottedCircle => '◌',
        CircleSymbol::CircleWithLeftHalfBlack => '◐',
        CircleSymbol::CircleWithRightHalfBlack => '◑',
        CircleSymbol::CircledDot => '◍',
        CircleSymbol::CircleWithVerticalFill => '◓',
        CircleSymbol::CircleWithHorizontalFill => '◒',
        CircleSymbol::Bullseye => '◎',
        CircleSymbol::SunSymbol => '☉',
        CircleSymbol::FishEye => '◉',
        CircleSymbol::CircleWithTwoDotsInside => '⚇',
        CircleSymbol::FilledCircleWithTwoDotsInside => '⚉',
        CircleSymbol::RedCircle => '🔴',
        CircleSymbol::BlueCircle => '🔵',
        CircleSymbol::CircledPlus => '⊕',
        CircleSymbol::CircledMinus => '⊖',
        CircleSymbol::CircledTimes => '⊗',
    }
}