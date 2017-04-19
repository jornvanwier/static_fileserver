use std::path::{Path, PathBuf};
use rocket::request::FromSegments;
use rocket::http::uri::*;
use std::ops::Deref;
use std::error::Error;
use std::fmt;
use std::fmt::{Formatter, Display};

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
    type Error = PathDirError;

    fn from_segments(segments: Segments<'a>) -> Result<PathDir, PathDirError> {
        let mut buf = PathBuf::new();
        for segment in segments {
            let decoded = URI::percent_decode(segment.as_bytes())
                .map_err(|e| PathDirError::Segment(SegmentError::Utf8(e)))?;

            if decoded == ".." {
                buf.pop();
            } else if decoded.starts_with('.') {
                return Err(PathDirError::Segment(SegmentError::BadStart('.')))
            } else if decoded.starts_with('*') {
                return Err(PathDirError::Segment(SegmentError::BadStart('*')))
            } else if decoded.ends_with(':') {
                return Err(PathDirError::Segment(SegmentError::BadEnd(':')))
            } else if decoded.ends_with('>') {
                return Err(PathDirError::Segment(SegmentError::BadEnd('>')))
            } else if decoded.ends_with('<') {
                return Err(PathDirError::Segment(SegmentError::BadEnd('<')))
            } else if decoded.contains('/') {
                return Err(PathDirError::Segment(SegmentError::BadChar('/')))
            } else if cfg!(windows) && decoded.contains('\\') {
                return Err(PathDirError::Segment(SegmentError::BadChar('\\')))
            } else {
                buf.push(&*decoded)
            }
        }

        buf = Path::new("files").join(buf);

        if !buf.is_dir() {
            return Err(PathDirError::NotDir)
        }

        Ok(PathDir{buffer: buf})
    }
}

#[derive(Debug)]
pub enum PathDirError {
    Segment(SegmentError),
    NotDir
}

impl Display for PathDirError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            PathDirError::Segment(ref e) => write!(f, "{:?}", e),
            PathDirError::NotDir => write!(f, "Not a directory")
        }
    }
}

impl Error for PathDirError {
    fn description(&self) -> &str {
        match *self {
            PathDirError::Segment(_) => "Invalid segment detected.",
            PathDirError::NotDir => "The provided does not point to a directory."
        }
    }
}