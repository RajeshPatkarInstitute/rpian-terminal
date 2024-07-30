/// Represents various arrow symbols available in Unicode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowSymbol {
    // Basic arrows
    LeftArrow,  // ← U+2190
    UpArrow,    // ↑ U+2191
    RightArrow, // → U+2192
    DownArrow,  // ↓ U+2193

    // Double arrows
    LeftDoubleArrow,  // ⇐ U+21D0
    UpDoubleArrow,    // ⇑ U+21D1
    RightDoubleArrow, // ⇒ U+21D2
    DownDoubleArrow,  // ⇓ U+21D3

    // Heavy arrows
    LeftHeavyArrow,  // ⟵ U+27F5
    UpHeavyArrow,    // ⟰ U+27F0
    RightHeavyArrow, // ⟶ U+27F6
    DownHeavyArrow,  // ⟱ U+27F1

    // Dashed arrows
    LeftDashedArrow,  // ⇠ U+21E0
    UpDashedArrow,    // ⇡ U+21E1
    RightDashedArrow, // ⇢ U+21E2
    DownDashedArrow,  // ⇣ U+21E3

    // Curved arrows
    LeftCurvedArrow,  // ↶ U+21B6
    UpCurvedArrow,    // ⤴ U+2934
    RightCurvedArrow, // ↷ U+21B7
    DownCurvedArrow,  // ⤵ U+2935

    // Diagonal arrows
    UpLeftArrow,    // ↖ U+2196
    UpRightArrow,   // ↗ U+2197
    DownRightArrow, // ↘ U+2198
    DownLeftArrow,  // ↙ U+2199

    // Special arrows
    LeftRightArrow,      // ↔ U+2194
    UpDownArrow,         // ↕ U+2195
    LeftwardsTailArrow,  // ↢ U+21A2
    RightwardsTailArrow, // ↣ U+21A3
    CircularArrow,       // ↻ U+21BB
}

/// Converts an ArrowSymbol to its corresponding Unicode character.
pub fn arrow_symbol_to_char(symbol: ArrowSymbol) -> char {
    match symbol {
        ArrowSymbol::LeftArrow => '←',
        ArrowSymbol::UpArrow => '↑',
        ArrowSymbol::RightArrow => '→',
        ArrowSymbol::DownArrow => '↓',
        ArrowSymbol::LeftDoubleArrow => '⇐',
        ArrowSymbol::UpDoubleArrow => '⇑',
        ArrowSymbol::RightDoubleArrow => '⇒',
        ArrowSymbol::DownDoubleArrow => '⇓',
        ArrowSymbol::LeftHeavyArrow => '⟵',
        ArrowSymbol::UpHeavyArrow => '⟰',
        ArrowSymbol::RightHeavyArrow => '⟶',
        ArrowSymbol::DownHeavyArrow => '⟱',
        ArrowSymbol::LeftDashedArrow => '⇠',
        ArrowSymbol::UpDashedArrow => '⇡',
        ArrowSymbol::RightDashedArrow => '⇢',
        ArrowSymbol::DownDashedArrow => '⇣',
        ArrowSymbol::LeftCurvedArrow => '↶',
        ArrowSymbol::UpCurvedArrow => '⤴',
        ArrowSymbol::RightCurvedArrow => '↷',
        ArrowSymbol::DownCurvedArrow => '⤵',
        ArrowSymbol::UpLeftArrow => '↖',
        ArrowSymbol::UpRightArrow => '↗',
        ArrowSymbol::DownRightArrow => '↘',
        ArrowSymbol::DownLeftArrow => '↙',
        ArrowSymbol::LeftRightArrow => '↔',
        ArrowSymbol::UpDownArrow => '↕',
        ArrowSymbol::LeftwardsTailArrow => '↢',
        ArrowSymbol::RightwardsTailArrow => '↣',
        ArrowSymbol::CircularArrow => '↻',
    }
}
