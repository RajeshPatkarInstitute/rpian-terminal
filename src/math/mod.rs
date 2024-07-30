/// Represents various mathematical symbols available in Unicode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathSymbol {
    // Basic operations
    Plus,     // + U+002B
    Minus,    // − U+2212
    Multiply, // × U+00D7
    Divide,   // ÷ U+00F7

    // Equality and inequality
    Equals,             // = U+003D
    NotEquals,          // ≠ U+2260
    LessThan,           // < U+003C
    GreaterThan,        // > U+003E
    LessThanOrEqual,    // ≤ U+2264
    GreaterThanOrEqual, // ≥ U+2265

    // Set theory
    ElementOf,    // ∈ U+2208
    NotElementOf, // ∉ U+2209
    Subset,       // ⊂ U+2282
    Superset,     // ⊃ U+2283
    Union,        // ∪ U+222A
    Intersection, // ∩ U+2229

    // Logic
    And,       // ∧ U+2227
    Or,        // ∨ U+2228
    Not,       // ¬ U+00AC
    Therefore, // ∴ U+2234
    Because,   // ∵ U+2235

    // Calculus
    PartialDerivative, // ∂ U+2202
    Integral,          // ∫ U+222B
    ContourIntegral,   // ∮ U+222E
    Infinity,          // ∞ U+221E

    // Geometry
    Degree,        // ° U+00B0
    Perpendicular, // ⟂ U+27C2
    Angle,         // ∠ U+2220
    MeasuredAngle, // ∡ U+2221

    // Greek letters (commonly used in math)
    Alpha, // α U+03B1
    Beta,  // β U+03B2
    Gamma, // γ U+03B3
    Delta, // δ U+03B4
    Pi,    // π U+03C0
    Sigma, // σ U+03C3

    // Other common symbols
    PlusMinus,    // ± U+00B1
    Sqrt,         // √ U+221A
    NthRoot,      // ∛ U+221B
    Dot,          // ⋅ U+22C5
    Proportional, // ∝ U+221D
}

/// Converts a MathSymbol to its corresponding Unicode character.
pub fn math_symbol_to_char(symbol: MathSymbol) -> char {
    match symbol {
        MathSymbol::Plus => '+',
        MathSymbol::Minus => '−',
        MathSymbol::Multiply => '×',
        MathSymbol::Divide => '÷',
        MathSymbol::Equals => '=',
        MathSymbol::NotEquals => '≠',
        MathSymbol::LessThan => '<',
        MathSymbol::GreaterThan => '>',
        MathSymbol::LessThanOrEqual => '≤',
        MathSymbol::GreaterThanOrEqual => '≥',
        MathSymbol::ElementOf => '∈',
        MathSymbol::NotElementOf => '∉',
        MathSymbol::Subset => '⊂',
        MathSymbol::Superset => '⊃',
        MathSymbol::Union => '∪',
        MathSymbol::Intersection => '∩',
        MathSymbol::And => '∧',
        MathSymbol::Or => '∨',
        MathSymbol::Not => '¬',
        MathSymbol::Therefore => '∴',
        MathSymbol::Because => '∵',
        MathSymbol::PartialDerivative => '∂',
        MathSymbol::Integral => '∫',
        MathSymbol::ContourIntegral => '∮',
        MathSymbol::Infinity => '∞',
        MathSymbol::Degree => '°',
        MathSymbol::Perpendicular => '⟂',
        MathSymbol::Angle => '∠',
        MathSymbol::MeasuredAngle => '∡',
        MathSymbol::Alpha => 'α',
        MathSymbol::Beta => 'β',
        MathSymbol::Gamma => 'γ',
        MathSymbol::Delta => 'δ',
        MathSymbol::Pi => 'π',
        MathSymbol::Sigma => 'σ',
        MathSymbol::PlusMinus => '±',
        MathSymbol::Sqrt => '√',
        MathSymbol::NthRoot => '∛',
        MathSymbol::Dot => '⋅',
        MathSymbol::Proportional => '∝',
    }
}
