use std::path::{Path, PathBuf};
use rocket::request::FromSegments;
use rocket::http::uri::*;
use std::ops::Deref;

pub struct PathDir {
    pub buffer: PathBuf
}

impl Deref for PathDir {
    type Target = PathBuf;

    fn deref(&self) -> &PathBuf {
        &self.buffer
    }
}

impl<'a> FromSegments<'a> for PathDir {
    type Error = SegmentError;

    fn from_segments(segments: Segments<'a>) -> Result<PathDir, SegmentError> {
        let mut buf = PathBuf::new();
        for segment in segments {
            let decoded = URI::percent_decode(segment.as_bytes())
                .map_err(|e| SegmentError::Utf8(e))?;

            if decoded == ".." {
                buf.pop();
            } else if decoded.starts_with('.') {
                return Err(SegmentError::BadStart('.'))
            } else if decoded.starts_with('*') {
                return Err(SegmentError::BadStart('*'))
            } else if decoded.ends_with(':') {
                return Err(SegmentError::BadEnd(':'))
            } else if decoded.ends_with('>') {
                return Err(SegmentError::BadEnd('>'))
            } else if decoded.ends_with('<') {
                return Err(SegmentError::BadEnd('<'))
            } else if decoded.contains('/') {
                return Err(SegmentError::BadChar('/'))
            } else if cfg!(windows) && decoded.contains('\\') {
                return Err(SegmentError::BadChar('\\'))
            } else {
                buf.push(&*decoded)
            }
        }

        buf = Path::new("files/").join(buf);

        if !buf.is_dir() {
            return Err(SegmentError::BadEnd('/'))
        }

        Ok(PathDir{buffer: buf})
    }
}