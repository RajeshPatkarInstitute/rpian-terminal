/// Represents various triangle symbols available in Unicode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriangleSymbol {
    // White triangles
    WhiteUpPointing,    // △ U+25B3
    WhiteDownPointing,  // ▽ U+25BD
    WhiteLeftPointing,  // ◁ U+25C1
    WhiteRightPointing, // ▷ U+25B7

    // Black triangles
    BlackUpPointing,    // ▲ U+25B2
    BlackDownPointing,  // ▼ U+25BC
    BlackLeftPointing,  // ◀ U+25C0
    BlackRightPointing, // ▶ U+25B6

    // Small triangles
    SmallBlackUpPointing,    // ▴ U+25B4
    SmallBlackDownPointing,  // ▾ U+25BE
    SmallBlackLeftPointing,  // ◂ U+25C2
    SmallBlackRightPointing, // ▸ U+25B8

    // Other triangles
    UpPointingTriangle,    // ▵ U+25B5
    RightPointingTriangle, // ▹ U+25B9
    DownPointingTriangle,  // ▿ U+25BF
    LeftPointingTriangle,  // ◃ U+25C3

    // Double triangles
    BlackUpPointingDouble,    // ⏶ U+23F6
    BlackDownPointingDouble,  // ⏷ U+23F7
    BlackLeftPointingDouble,  // ⏴ U+23F4
    BlackRightPointingDouble, // ⏵ U+23F5
}

/// Converts a TriangleSymbol to its corresponding Unicode character.
pub fn triangle_symbol_to_char(symbol: TriangleSymbol) -> char {
    match symbol {
        TriangleSymbol::WhiteUpPointing => '△',
        TriangleSymbol::WhiteDownPointing => '▽',
        TriangleSymbol::WhiteLeftPointing => '◁',
        TriangleSymbol::WhiteRightPointing => '▷',
        TriangleSymbol::BlackUpPointing => '▲',
        TriangleSymbol::BlackDownPointing => '▼',
        TriangleSymbol::BlackLeftPointing => '◀',
        TriangleSymbol::BlackRightPointing => '▶',
        TriangleSymbol::SmallBlackUpPointing => '▴',
        TriangleSymbol::SmallBlackDownPointing => '▾',
        TriangleSymbol::SmallBlackLeftPointing => '◂',
        TriangleSymbol::SmallBlackRightPointing => '▸',
        TriangleSymbol::UpPointingTriangle => '▵',
        TriangleSymbol::RightPointingTriangle => '▹',
        TriangleSymbol::DownPointingTriangle => '▿',
        TriangleSymbol::LeftPointingTriangle => '◃',
        TriangleSymbol::BlackUpPointingDouble => '⏶',
        TriangleSymbol::BlackDownPointingDouble => '⏷',
        TriangleSymbol::BlackLeftPointingDouble => '⏴',
        TriangleSymbol::BlackRightPointingDouble => '⏵',
    }
}
