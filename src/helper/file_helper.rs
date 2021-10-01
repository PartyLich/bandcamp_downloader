use std::path::PathBuf;

use chrono::Datelike;
use regex::Regex;

use crate::model::Album;

#[cfg(debug_assertions)]
pub fn get_root_dir() -> PathBuf {
    env!("CARGO_MANIFEST_DIR").into()
}

#[cfg(not(debug_assertions))]
pub fn get_root_dir() -> PathBuf {
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop();
        exe_path
    } else {
        PathBuf::new()
    }
}

/// Replaces the forbidden characters \ / : * ? " &lt; &gt; | from with an underscore _
/// in order to be used for a Windows file or folder.
// Windows rules: https://docs.microsoft.com/en-us/windows/desktop/FileIO/naming-a-file
pub fn sanitize_file_name(file_name: &str) -> String {
    lazy_static! {
        static ref TRAIL_DOTS: Regex = regex::Regex::new(r"\.+$").unwrap();
        static ref WHITESPACE: Regex = regex::Regex::new(r"\s+").unwrap();
        static ref RESERVED_CHARS: Regex = regex::Regex::new(r#"[\\/:*?"<>|]"#).unwrap();
    }

    // Replace reserved characters with '_'
    let file_name = RESERVED_CHARS.replace_all(file_name, "_");

    // Remove trailing dot(s)
    let file_name = TRAIL_DOTS.replace(&file_name, "");

    // Replace whitespace(s) with ' '
    let file_name = WHITESPACE.replace_all(&file_name, " ");

    // Remove trailing whitespace
    file_name.trim_end().to_string()
}

/// Returns the file name to be used for the item from the provided file name format, by
/// replacing the placeholders strings with their corresponding values.
/// The returned file name DOES NOT contain the extension.
pub fn parse_filename(filename_format: &str, album: &Album) -> String {
    let file_name = filename_format
        .replace("{year}", &album.release_date.year().to_string())
        .replace("{month}", &format!("{:02}", album.release_date.month()))
        .replace("{day}", &format!("{:02}", album.release_date.day()))
        .replace("{album}", &album.title)
        .replace("{artist}", &album.artist);

    sanitize_file_name(&file_name)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_filename() {
        let msg = "should replace reserved chars with '_'";
        let expected = "Foo_________Bar";
        let actual = sanitize_file_name(r#"Foo?*/\|<>:"Bar   ..."#);
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should replace consecutive whitespace chars (including newlines) with ' '";
        let expected = "Foo Bar";
        let actual = sanitize_file_name(
            r#"Foo

  Bar   ..."#,
        );
        assert_eq!(actual, expected, "{}", msg);
    }
}
