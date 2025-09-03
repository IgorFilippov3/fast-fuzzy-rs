use unicode_normalization::UnicodeNormalization;

/// Normalizes a string by:
/// - Removing diacritical marks (accents).
/// - Optionally converting to lowercase.
/// - Normalizing whitespace (multiple spaces, tabs, newlines → single space).
///
/// This function uses **Unicode NFD decomposition** to separate base characters
/// from diacritics and then filters out combining marks.
///
/// # Arguments
///
/// * `input` - The input string to normalize.
/// * `to_lowercase` - If `true`, the result is converted to lowercase.
///
/// # Examples
///
/// ```
/// use mylib::normalize_string;
///
/// assert_eq!(normalize_string("Café", true), "cafe");
/// assert_eq!(normalize_string("  Hello\tWorld\n", false), "Hello World");
/// ```
pub fn normalize_string(input: &str, to_lowercase: bool) -> String {
    let mut result = input
        .nfd()
        .filter(|c| !is_combining_mark(*c))
        .collect::<String>();

    if to_lowercase {
        result = result.to_lowercase();
    }

    normalize_whitespace(&result)
}

/// Internal helper for detecting whether a character is a Unicode combining mark
fn is_combining_mark(c: char) -> bool {
    match c {
        '\u{0300}'..='\u{036F}' | // Combining Diacritical Marks
        '\u{1AB0}'..='\u{1AFF}' | // Combining Diacritical Marks Extended
        '\u{1DC0}'..='\u{1DFF}' | // Combining Diacritical Marks Supplement
        '\u{20D0}'..='\u{20FF}' | // Combining Diacritical Marks for Symbols
        '\u{FE20}'..='\u{FE2F}'   // Combining Half Marks
        => true,
        _ => false,
    }
}

/// Internal helper for collapsing multiple whitespace characters into a single space.
fn normalize_whitespace(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_string() {
        assert_eq!(normalize_string("Café", true), "cafe");
        assert_eq!(normalize_string("naïve", true), "naive");
        assert_eq!(normalize_string("résumé", true), "resume");
    }
}
