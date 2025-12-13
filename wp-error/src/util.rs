use unicode_segmentation::UnicodeSegmentation;
/// Return the first 30 Unicode grapheme clusters of the given string.
/// This is a safe, user-facing truncation (not a logical 'split').
pub fn split_string(s: &str) -> String {
    s.graphemes(true).take(30).collect()
}
