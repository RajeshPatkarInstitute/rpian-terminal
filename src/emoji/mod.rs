/// Represents various smiley face symbols available in Unicode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmojiSymbol {
    // Basic smileys
    HappyFace,    // ☺ U+263A
    SmilingFace,  // 😊 U+1F60A
    GrinningFace, // 😀 U+1F600
    LaughingFace, // 😄 U+1F604
    TearsOfJoy,   // 😂 U+1F602
    WinkingFace,  // 😉 U+1F609
    SmilingEyes,  // 😊 U+1F60A

    // Negative emotions
    SadFace,              // ☹ U+2639
    SlightlyFrowningFace, // 🙁 U+1F641
    FrowningFace,         // 😦 U+1F626
    CryingFace,           // 😢 U+1F622
    LoudlyCryingFace,     // 😭 U+1F62D
    AngryFace,            // 😠 U+1F620
    PoutingFace,          // 😡 U+1F621

    // Neutral or ambiguous
    NeutralFace,        // 😐 U+1F610
    ExpressionlessFace, // 😑 U+1F611
    ConfusedFace,       // 😕 U+1F615
    ThinkingFace,       // 🤔 U+1F914
    ZipperMouthFace,    // 🤐 U+1F910

    // Playful or silly
    StuckOutTongue, // 😛 U+1F61B
    WinkingTongue,  // 😜 U+1F61C
    Zany,           // 🤪 U+1F92A

    // Sleep-related
    SleepyFace,   // 😪 U+1F62A
    SleepingFace, // 😴 U+1F634

    // Other
    NerdFace,      // 🤓 U+1F913
    CowboyHatFace, // 🤠 U+1F920
    ClownFace,     // 🤡 U+1F921
    Alien,         // 👽 U+1F47D
    Robot,         // 🤖 U+1F916
}

/// Converts a SmileySymbol to its corresponding Unicode character.
pub fn smiley_symbol_to_char(symbol: EmojiSymbol) -> char {
    match symbol {
        EmojiSymbol::HappyFace => '☺',
        EmojiSymbol::SmilingFace => '😊',
        EmojiSymbol::GrinningFace => '😀',
        EmojiSymbol::LaughingFace => '😄',
        EmojiSymbol::TearsOfJoy => '😂',
        EmojiSymbol::WinkingFace => '😉',
        EmojiSymbol::SmilingEyes => '😊',
        EmojiSymbol::SadFace => '☹',
        EmojiSymbol::SlightlyFrowningFace => '🙁',
        EmojiSymbol::FrowningFace => '😦',
        EmojiSymbol::CryingFace => '😢',
        EmojiSymbol::LoudlyCryingFace => '😭',
        EmojiSymbol::AngryFace => '😠',
        EmojiSymbol::PoutingFace => '😡',
        EmojiSymbol::NeutralFace => '😐',
        EmojiSymbol::ExpressionlessFace => '😑',
        EmojiSymbol::ConfusedFace => '😕',
        EmojiSymbol::ThinkingFace => '🤔',
        EmojiSymbol::ZipperMouthFace => '🤐',
        EmojiSymbol::StuckOutTongue => '😛',
        EmojiSymbol::WinkingTongue => '😜',
        EmojiSymbol::Zany => '🤪',
        EmojiSymbol::SleepyFace => '😪',
        EmojiSymbol::SleepingFace => '😴',
        EmojiSymbol::NerdFace => '🤓',
        EmojiSymbol::CowboyHatFace => '🤠',
        EmojiSymbol::ClownFace => '🤡',
        EmojiSymbol::Alien => '👽',
        EmojiSymbol::Robot => '🤖',
    }
}
