/// Represents various star symbols available in Unicode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarSymbol {
    // Basic stars
    BlackStar, // ★ U+2605
    WhiteStar, // ☆ U+2606

    // Pointed stars
    FourPointedBlackStar,  // ✦ U+2726
    FourPointedWhiteStar,  // ✧ U+2727
    FivePointedBlackStar,  // ✭ U+272D
    FivePointedWhiteStar,  // ✮ U+272E
    SixPointedBlackStar,   // ✶ U+2736
    SixPointedWhiteStar,   // ✴ U+2734
    EightPointedBlackStar, // ✴ U+2734
    EightPointedWhiteStar, // ✵ U+2735

    // Special stars
    CircledWhiteStar,      // ✪ U+272A
    CircledBlackStar,      // ✫ U+272B
    OpenCenterBlackStar,   // ✯ U+272F
    HeavyEightPointedStar, // ✷ U+2737
    SparklingStar,         // ❈ U+2748

    // Astronomical symbols
    SunStar,  // ☀ U+2600
    Asterisk, // ✱ U+2731

    // Additional common stars
    BoldFivePointedBlackStar, // ⭐ U+2B50
    OutlinedBlackStar,        // ✰ U+2730
    HeavyFourBalloonStar,     // ✣ U+2723
}

/// Converts a StarSymbol to its corresponding Unicode character.
pub fn star_symbol_to_char(symbol: StarSymbol) -> char {
    match symbol {
        StarSymbol::BlackStar => '★',
        StarSymbol::WhiteStar => '☆',
        StarSymbol::FourPointedBlackStar => '✦',
        StarSymbol::FourPointedWhiteStar => '✧',
        StarSymbol::FivePointedBlackStar => '✭',
        StarSymbol::FivePointedWhiteStar => '✮',
        StarSymbol::SixPointedBlackStar => '✶',
        StarSymbol::SixPointedWhiteStar => '✴',
        StarSymbol::EightPointedBlackStar => '✴',
        StarSymbol::EightPointedWhiteStar => '✵',
        StarSymbol::CircledWhiteStar => '✪',
        StarSymbol::CircledBlackStar => '✫',
        StarSymbol::OpenCenterBlackStar => '✯',
        StarSymbol::HeavyEightPointedStar => '✷',
        StarSymbol::SparklingStar => '❈',
        StarSymbol::SunStar => '☀',
        StarSymbol::Asterisk => '✱',
        StarSymbol::BoldFivePointedBlackStar => '⭐',
        StarSymbol::OutlinedBlackStar => '✰',
        StarSymbol::HeavyFourBalloonStar => '✣',
    }
}
