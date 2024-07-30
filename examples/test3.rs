#[allow(non_camel_case_types)]
/// Represents various Unicode block elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockElement {
    Full,
    UpperHalf,
    LowerHalf,
    LeftHalf,
    RightHalf,
    LightShade,
    MediumShade,
    DarkShade,
    UpperOneEighth,
    UpperOneQuarter,
    UpperThreeEighths,
    UpperFiveEighths,
    UpperThreeQuarters,
    UpperSevenEighths,
    LowerOneEighth,
    LowerOneQuarter,
    LowerThreeEighths,
    LowerFiveEighths,
    LowerThreeQuarters,
    LowerSevenEighths,
    LeftOneEighth,
    LeftOneQuarter,
    LeftThreeEighths,
    LeftFiveEighths,
    LeftThreeQuarters,
    LeftSevenEighths,
    RightOneEighth,
    RightOneQuarter,
    RightThreeEighths,
    RightFiveEighths,
    RightThreeQuarters,
    RightSevenEighths,
    Quadrant_UpperLeft,
    Quadrant_UpperRight,
    Quadrant_LowerLeft,
    Quadrant_LowerRight,
}

impl BlockElement {
    /// Convert the BlockElement to its corresponding Unicode character.
    pub fn to_char(self) -> char {
        match self {
            Self::Full => '\u{2588}',
            Self::UpperHalf => '\u{2580}',
            Self::LowerHalf => '\u{2584}',
            Self::LeftHalf => '\u{258C}',
            Self::RightHalf => '\u{2590}',
            Self::LightShade => '\u{2591}',
            Self::MediumShade => '\u{2592}',
            Self::DarkShade => '\u{2593}',
            Self::UpperOneEighth => '\u{2594}',
            Self::UpperOneQuarter => '\u{2595}',
            Self::UpperThreeEighths => '\u{2596}',
            Self::UpperFiveEighths => '\u{2597}',
            Self::UpperThreeQuarters => '\u{2598}',
            Self::UpperSevenEighths => '\u{2599}',
            Self::LowerOneEighth => '\u{259A}',
            Self::LowerOneQuarter => '\u{259B}',
            Self::LowerThreeEighths => '\u{259C}',
            Self::LowerFiveEighths => '\u{259D}',
            Self::LowerThreeQuarters => '\u{259E}',
            Self::LowerSevenEighths => '\u{259F}',
            Self::LeftOneEighth => '\u{258F}',
            Self::LeftOneQuarter => '\u{258E}',
            Self::LeftThreeEighths => '\u{258D}',
            Self::LeftFiveEighths => '\u{258B}',
            Self::LeftThreeQuarters => '\u{258A}',
            Self::LeftSevenEighths => '\u{2589}',
            Self::RightOneEighth => '\u{2595}',
            Self::RightOneQuarter => '\u{2596}',
            Self::RightThreeEighths => '\u{2597}',
            Self::RightFiveEighths => '\u{2598}',
            Self::RightThreeQuarters => '\u{2599}',
            Self::RightSevenEighths => '\u{259A}',
            Self::Quadrant_UpperLeft => '\u{2598}',
            Self::Quadrant_UpperRight => '\u{259D}',
            Self::Quadrant_LowerLeft => '\u{2596}',
            Self::Quadrant_LowerRight => '\u{2597}',
        }
    }

    /// Convert the BlockElement to its UTF-8 byte representation.
    pub fn to_utf8_bytes(self) -> [u8; 3] {
        let c = self.to_char();
        let mut bytes = [0; 3];
        c.encode_utf8(&mut bytes);
        bytes
    }
}

impl std::fmt::Display for BlockElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

// Example usage function
pub fn demonstrate_block_elements() {
    println!("Block Elements Demonstration:");
    for element in [
        BlockElement::Full,
        BlockElement::UpperHalf,
        BlockElement::LowerHalf,
        BlockElement::LeftHalf,
        BlockElement::RightHalf,
        BlockElement::LightShade,
        BlockElement::MediumShade,
        BlockElement::DarkShade,
        BlockElement::Quadrant_UpperLeft,
        BlockElement::Quadrant_UpperRight,
        BlockElement::Quadrant_LowerLeft,
        BlockElement::Quadrant_LowerRight,
    ]
    .iter()
    {
        println!(
            "{:?}: {} (UTF-8: {:?})",
            element,
            element,
            element.to_utf8_bytes()
        );
    }

    // Example: Creating a simple progress bar
    let width = 20;
    let progress = 0.7; // 70% progress
    let filled = (width as f32 * progress) as usize;
    let bar: String = BlockElement::Full.to_string().repeat(filled)
        + &BlockElement::LightShade.to_string().repeat(width - filled);
    println!("\nProgress bar example:");
    println!("[{}] {:.0}%", bar, progress * 100.0);
}

fn main() {
    demonstrate_block_elements();
}
