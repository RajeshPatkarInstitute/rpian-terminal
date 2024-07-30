/// Represents Braille symbols available in Unicode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrailleSymbol {
    Blank,     // ⠀ U+2800
    Dot1,      // ⠁ U+2801
    Dot2,      // ⠂ U+2802
    Dot12,     // ⠃ U+2803
    Dot3,      // ⠄ U+2804
    Dot13,     // ⠅ U+2805
    Dot23,     // ⠆ U+2806
    Dot123,    // ⠇ U+2807
    Dot4,      // ⠈ U+2808
    Dot14,     // ⠉ U+2809
    Dot24,     // ⠊ U+280A
    Dot124,    // ⠋ U+280B
    Dot34,     // ⠌ U+280C
    Dot134,    // ⠍ U+280D
    Dot234,    // ⠎ U+280E
    Dot1234,   // ⠏ U+280F
    Dot5,      // ⠐ U+2810
    Dot15,     // ⠑ U+2811
    Dot25,     // ⠒ U+2812
    Dot125,    // ⠓ U+2813
    Dot35,     // ⠔ U+2814
    Dot135,    // ⠕ U+2815
    Dot235,    // ⠖ U+2816
    Dot1235,   // ⠗ U+2817
    Dot45,     // ⠘ U+2818
    Dot145,    // ⠙ U+2819
    Dot245,    // ⠚ U+281A
    Dot1245,   // ⠛ U+281B
    Dot345,    // ⠜ U+281C
    Dot1345,   // ⠝ U+281D
    Dot2345,   // ⠞ U+281E
    Dot12345,  // ⠟ U+281F
    Dot6,      // ⠠ U+2820
    Dot16,     // ⠡ U+2821
    Dot26,     // ⠢ U+2822
    Dot126,    // ⠣ U+2823
    Dot36,     // ⠤ U+2824
    Dot136,    // ⠥ U+2825
    Dot236,    // ⠦ U+2826
    Dot1236,   // ⠧ U+2827
    Dot46,     // ⠨ U+2828
    Dot146,    // ⠩ U+2829
    Dot246,    // ⠪ U+282A
    Dot1246,   // ⠫ U+282B
    Dot346,    // ⠬ U+282C
    Dot1346,   // ⠭ U+282D
    Dot2346,   // ⠮ U+282E
    Dot12346,  // ⠯ U+282F
    Dot56,     // ⠰ U+2830
    Dot156,    // ⠱ U+2831
    Dot256,    // ⠲ U+2832
    Dot1256,   // ⠳ U+2833
    Dot356,    // ⠴ U+2834
    Dot1356,   // ⠵ U+2835
    Dot2356,   // ⠶ U+2836
    Dot12356,  // ⠷ U+2837
    Dot456,    // ⠸ U+2838
    Dot1456,   // ⠹ U+2839
    Dot2456,   // ⠺ U+283A
    Dot12456,  // ⠻ U+283B
    Dot3456,   // ⠼ U+283C
    Dot13456,  // ⠽ U+283D
    Dot23456,  // ⠾ U+283E
    Dot123456, // ⠿ U+283F
}

impl BrailleSymbol {
    /// Converts the BrailleSymbol enum variant to its corresponding Unicode character.
    pub fn to_char(self) -> char {
        match self {
            BrailleSymbol::Blank => '⠀',
            BrailleSymbol::Dot1 => '⠁',
            BrailleSymbol::Dot2 => '⠂',
            BrailleSymbol::Dot12 => '⠃',
            BrailleSymbol::Dot3 => '⠄',
            BrailleSymbol::Dot13 => '⠅',
            BrailleSymbol::Dot23 => '⠆',
            BrailleSymbol::Dot123 => '⠇',
            BrailleSymbol::Dot4 => '⠈',
            BrailleSymbol::Dot14 => '⠉',
            BrailleSymbol::Dot24 => '⠊',
            BrailleSymbol::Dot124 => '⠋',
            BrailleSymbol::Dot34 => '⠌',
            BrailleSymbol::Dot134 => '⠍',
            BrailleSymbol::Dot234 => '⠎',
            BrailleSymbol::Dot1234 => '⠏',
            BrailleSymbol::Dot5 => '⠐',
            BrailleSymbol::Dot15 => '⠑',
            BrailleSymbol::Dot25 => '⠒',
            BrailleSymbol::Dot125 => '⠓',
            BrailleSymbol::Dot35 => '⠔',
            BrailleSymbol::Dot135 => '⠕',
            BrailleSymbol::Dot235 => '⠖',
            BrailleSymbol::Dot1235 => '⠗',
            BrailleSymbol::Dot45 => '⠘',
            BrailleSymbol::Dot145 => '⠙',
            BrailleSymbol::Dot245 => '⠚',
            BrailleSymbol::Dot1245 => '⠛',
            BrailleSymbol::Dot345 => '⠜',
            BrailleSymbol::Dot1345 => '⠝',
            BrailleSymbol::Dot2345 => '⠞',
            BrailleSymbol::Dot12345 => '⠟',
            BrailleSymbol::Dot6 => '⠠',
            BrailleSymbol::Dot16 => '⠡',
            BrailleSymbol::Dot26 => '⠢',
            BrailleSymbol::Dot126 => '⠣',
            BrailleSymbol::Dot36 => '⠤',
            BrailleSymbol::Dot136 => '⠥',
            BrailleSymbol::Dot236 => '⠦',
            BrailleSymbol::Dot1236 => '⠧',
            BrailleSymbol::Dot46 => '⠨',
            BrailleSymbol::Dot146 => '⠩',
            BrailleSymbol::Dot246 => '⠪',
            BrailleSymbol::Dot1246 => '⠫',
            BrailleSymbol::Dot346 => '⠬',
            BrailleSymbol::Dot1346 => '⠭',
            BrailleSymbol::Dot2346 => '⠮',
            BrailleSymbol::Dot12346 => '⠯',
            BrailleSymbol::Dot56 => '⠰',
            BrailleSymbol::Dot156 => '⠱',
            BrailleSymbol::Dot256 => '⠲',
            BrailleSymbol::Dot1256 => '⠳',
            BrailleSymbol::Dot356 => '⠴',
            BrailleSymbol::Dot1356 => '⠵',
            BrailleSymbol::Dot2356 => '⠶',
            BrailleSymbol::Dot12356 => '⠷',
            BrailleSymbol::Dot456 => '⠸',
            BrailleSymbol::Dot1456 => '⠹',
            BrailleSymbol::Dot2456 => '⠺',
            BrailleSymbol::Dot12456 => '⠻',
            BrailleSymbol::Dot3456 => '⠼',
            BrailleSymbol::Dot13456 => '⠽',
            BrailleSymbol::Dot23456 => '⠾',
            BrailleSymbol::Dot123456 => '⠿',
        }
    }
}
