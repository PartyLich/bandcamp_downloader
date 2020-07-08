use regex::Regex;

use crate::{
    error::Error,
    Result,
};

/// Get the TralbumData content from the page
fn get_album_data(raw_html: &str) -> Result<String> {
    lazy_static! {
        static ref ALBUM_DATA_RE: Regex =
            regex::Regex::new(r"(?s)var TralbumData = (?P<data>\{.*?\});").unwrap();
        static ref COMMENTS_RE: Regex = Regex::new(r"(?m)^\s*?//.*$").unwrap();
        static ref TAIL_COMMENTS_RE: Regex = Regex::new(r"(?m)^(?:.*\s*?)(\s//.*)$").unwrap();
        static ref QUOTE_KEYS_RE: Regex =
            Regex::new(r"(?m)^(?P<whitespace>\s*)(?P<key>\w*?):").unwrap();
    }

    let album_data = ALBUM_DATA_RE
        .captures(raw_html)
        .ok_or(Error::NoAlbumData)?
        .name("data")
        .unwrap()
        .as_str();

    // using a js object that has unquoted keys, comments, etc
    let album_data = COMMENTS_RE.replace_all(album_data, "");
    let album_data = TAIL_COMMENTS_RE.replace_all(&album_data, "");
    let album_data = QUOTE_KEYS_RE.replace_all(&album_data, r#"${whitespace}"${key}":"#);

    Ok(album_data.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gets_album_data() {
        let raw_html = r#" var TralbumData = {
            foo: "bar"
        };

        var baz = {
            wombo: "combo"
        };"#;

        let expected = r#"{
            "foo": "bar"
        }"#;
        let actual = get_album_data(&raw_html).unwrap();
        assert_eq!(actual, expected);
    }
}
