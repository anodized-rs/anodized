use std::collections::HashMap;

use crop::Rope;
use proc_macro2::LineColumn;

/// Extract standalone comments (comments on their own line, no code before them).
///
/// Uses a scan_gap-like approach but only extracts comments that are on their own line.
/// This avoids issues with inline comments and comments inside nested structures
/// which would be lost during prettyplease formatting.
///
/// Returns a HashMap mapping line numbers (0-indexed) to optional comment text:
/// - Some(comment_text) for lines with standalone comments
/// - None for empty lines (preserves vertical spacing)
pub(crate) fn extract_standalone_comments(
    source: &str,
    start_line: usize,
) -> HashMap<usize, Option<String>> {
    let mut result = HashMap::new();

    for (idx, line) in source.lines().enumerate() {
        let line_index = start_line + idx;
        let trimmed = line.trim();

        // Check if line is a standalone comment (only whitespace before //)
        if let Some(comment_text) = trimmed.strip_prefix("//") {
            let comment_text = comment_text.trim().to_string();
            if !comment_text.is_empty() {
                result.insert(line_index, Some(comment_text));
            } else {
                // Empty comment line
                result.insert(line_index, None);
            }
        } else if trimmed.is_empty() {
            // Empty line (no comment, no code)
            result.insert(line_index, None);
        }
    }

    result
}

/// Check if source text has inline comments (comments after code on the same line)
/// or comments inside block/paren delimiters.
///
/// Comments inside `[]` brackets are allowed — those are handled by the formatter's
/// comment-aware array formatting. Comments inside `{}` or `()` are problematic
/// because prettyplease strips them when reformatting expressions.
pub(crate) fn has_problematic_comments(source: &str) -> bool {
    for line in source.lines() {
        if let Some((before, _after)) = line.split_once("//") {
            let before_trimmed = before.trim();
            if !before_trimmed.is_empty() && !before_trimmed.starts_with("//") {
                return true;
            }
        }
    }

    let mut block_depth: i32 = 0;

    for line in source.lines() {
        let trimmed = line.trim();

        let code_part = trimmed
            .split_once("//")
            .map_or(trimmed, |(before, _)| before);

        for ch in code_part.chars() {
            match ch {
                '{' => block_depth += 1,
                '}' => block_depth = block_depth.saturating_sub(1),
                _ => {}
            }
        }

        if block_depth > 0 && trimmed.starts_with("//") {
            return true;
        }
    }

    false
}

/// Convert a LineColumn position to a byte offset in the Rope.
pub fn line_column_to_byte(source: &Rope, point: LineColumn) -> usize {
    let line_byte = source.byte_of_line(point.line - 1);
    let line = source.line(point.line - 1);
    let char_byte: usize = line.chars().take(point.column).map(|c| c.len_utf8()).sum();
    line_byte + char_byte
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_standalone_comments() {
        let source = "requires: x > 0,\n// This is a comment\nensures: *output > 0";
        let comments = extract_standalone_comments(source, 0);

        assert_eq!(
            comments.get(&1),
            Some(&Some("This is a comment".to_string()))
        );
        assert!(!comments.contains_key(&0)); // Code line, not a comment
        assert!(!comments.contains_key(&2)); // Code line, not a comment
    }

    #[test]
    fn test_extract_multiple_comments_and_empty_lines() {
        let source = "// First comment\n\n// Second comment\nrequires: x > 0";
        let comments = extract_standalone_comments(source, 5);

        assert_eq!(comments.get(&5), Some(&Some("First comment".to_string())));
        assert_eq!(comments.get(&6), Some(&None)); // Empty line
        assert_eq!(comments.get(&7), Some(&Some("Second comment".to_string())));
        assert!(!comments.contains_key(&8)); // Code line
    }

    #[test]
    fn test_has_problematic_comments_inline() {
        let source = "requires: x > 0, // inline comment\nensures: *output > 0";
        assert!(has_problematic_comments(source));
    }

    #[test]
    fn test_has_problematic_comments_in_block() {
        let source = "requires: {\n    // comment in block\n    x > 0\n}";
        assert!(has_problematic_comments(source));
    }

    #[test]
    fn test_has_no_problematic_comments_in_array() {
        let source = "requires: [\n    // comment in array\n    x > 0,\n    y > 0\n]";
        assert!(!has_problematic_comments(source));
    }

    #[test]
    fn test_has_no_problematic_comments_standalone() {
        let source =
            "// standalone comment\nrequires: x > 0,\n// another standalone\nensures: *output > 0";
        assert!(!has_problematic_comments(source));
    }
}
