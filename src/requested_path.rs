use iron::Request;
use std::path::{Path, PathBuf};
use std::fs::PathExt;

pub struct RequestedPath {
    pub path: PathBuf,
}

impl RequestedPath {
    pub fn new<P: AsRef<Path>>(root_path: P, request: &Request) -> RequestedPath {
        let mut path = root_path.as_ref().to_path_buf();

        path.extend(&request.url.path);

        RequestedPath { path: path }
    }

    pub fn should_redirect(&self, request: &Request) -> bool {
        let last_url_element = request.url.path[..]
            .last()
            .map(|s| &s[..]);

        // As per servo/rust-url/serialize_path, URLs ending in a slash have an
        // empty string stored as the last component of their path. Rust-url
        // even ensures that url.path is non-empty by appending a forward slash
        // to URLs like http://example.com
        // Some middleware may mutate the URL's path to violate this property,
        // so the empty list case is handled as a redirect.
        let has_trailing_slash = match last_url_element {
            Some("") => true,
            _ => false,
        };

        self.path.is_dir() && !has_trailing_slash
    }

    pub fn get_file(self) -> Option<PathBuf> {
        if self.path.is_file() {
            return Some(self.path);
        }

        let index_path = self.path.join("index.html");

        if index_path.is_file() {
            Some(index_path)
        } else {
            None
        }
    }
}
